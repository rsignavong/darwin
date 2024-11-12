use super::msg::ReconciliationMsg;
use crate::consumers::{AnonymizationsConsumer, RawRecordsConsumer, ReconciliationProcessConsumer};
use crate::decoders::{ReconciliationDataBody, ReconciliationDataMetadata};
use crate::processors::{StatusMsg, StatusProcessorSender};
use crate::producers::{ReconciliationDataProducerSender, RecordsProducer, RecordsProducerSender};
use crate::resources::{ProcessorId, ResourcesError};
use crate::resources::{ProcessorTopic, Reconciliation, StatusReconciliation, StatusState};
use crate::resources::{ReconciliationData, ReconciliationDataRecord, ReconciliationDataTopics};
use anyhow::{anyhow, bail, Error as AnyError};
use data_stream::consumer::{Consumer, ConsumerEvent};
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder, StreamProcessor};
use data_stream::processor::{ProcessorMessage, ProcessorSender, ProcessorThread};
use data_stream::producer::{Producer, ProducerMessage};
use data_stream::stream::{InputEvent, InputMessage, OutputMessage, StreamNext};
use derive_new::new;
use futures::future::{try_join, try_join_all};
use log::info;
use smol::block_on;
use std::sync::Arc;

const NAME: &'static str = "ReconciliationProcessor";

pub type ReconciliationProcessorSender = ProcessorSender<ReconciliationMsg>;

#[derive(new)]
pub struct ReconciliationProcessor {
    processor_id: Arc<ProcessorId>,
    #[new(default)]
    anonymizations_topics: Arc<Vec<ProcessorTopic>>,
    reconciliation: Reconciliation,
    #[new(default)]
    raw_records_topics: Arc<Vec<ProcessorTopic>>,
    #[new(default)]
    reconciliation_processor_sender: Option<ReconciliationProcessorSender>,
    reconciliation_data_producer_sender: ReconciliationDataProducerSender,
    #[new(default)]
    record_producer_sender: Option<RecordsProducerSender>,
    status_processor_sender: StatusProcessorSender,
}

impl StreamProcessor<ReconciliationMsg> for ReconciliationProcessor {
    fn config(&self, builder: &mut ProcessorConfigBuilder) -> Result<ProcessorConfig, AnyError> {
        builder.name(NAME).build().map_err(|e| anyhow!(e))
    }

    fn created(&mut self, message: &ReconciliationMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            ReconciliationMsg::Activate(reconciliation_process) => {
                if let Some(mut reconciliation_data_records) =
                    self.reconciliation.set_mapping(reconciliation_process)?
                {
                    let topics = ReconciliationDataTopics::new(
                        reconciliation_process.anonymizations_topics.clone(),
                        reconciliation_process.raw_records_topics.clone(),
                        reconciliation_process.reconciliations_records_topic.clone(),
                    );

                    self.setup_topics_and_spawn_reconciliation_producer(&topics)?;

                    reconciliation_data_records.push(ReconciliationDataRecord::new(
                        ReconciliationData::Topics(topics),
                        self.processor_id.clone(),
                    ));
                    block_on(async {
                        let sends: Vec<_> = reconciliation_data_records
                            .into_iter()
                            .map(|record| {
                                self.reconciliation_data_producer_sender
                                    .send(ProducerMessage::Output(OutputMessage::new(record, None)))
                            })
                            .collect();
                        try_join_all(sends).await?;

                        Ok::<(), AnyError>(())
                    })?;
                }
                next.next();
                self.spawn_records_consumers()?;
                Ok(())
            }

            ReconciliationMsg::Anonymize(anonymization_body) => {
                self.reconciliation.anonymize_profile(anonymization_body)?;
                next.next();
                Ok(())
            }

            ReconciliationMsg::Match(raw_record_body, raw_record_metadata) => {
                if let Some((record, reconciliation_data_records)) = self
                    .reconciliation
                    .match_record(raw_record_body, raw_record_metadata)?
                {
                    if let Some(ref record_producer_sender) = self.record_producer_sender {
                        block_on(async {
                            self.status_processor_sender
                                .send(ProcessorMessage::Input(InputMessage::new(
                                    InputEvent::Custom,
                                    StatusMsg::Counts(
                                        record.profiles_count.clone(),
                                        record.reconciliations_count.clone(),
                                    ),
                                    None,
                                )))
                                .await?;

                            let sends: Vec<_> = reconciliation_data_records
                                .into_iter()
                                .map(|record| {
                                    self.reconciliation_data_producer_sender.send(
                                        ProducerMessage::Output(OutputMessage::new(record, None)),
                                    )
                                })
                                .collect();
                            try_join_all(sends).await?;

                            record_producer_sender
                                .send(ProducerMessage::Output(OutputMessage::new(
                                    record,
                                    Some(next.clone()),
                                )))
                                .await?;
                            Ok::<(), AnyError>(())
                        })?;
                    } else {
                        bail!(ResourcesError::ReconciliationMatchMissingRecordsProducerTopic);
                    }
                } else {
                    next.next();
                }
                Ok(())
            }

            ReconciliationMsg::ReloadState(
                reconciliation_data_body,
                reconciliation_data_metadata,
            ) => {
                self.set_state(
                    reconciliation_data_body,
                    reconciliation_data_metadata,
                    false,
                )?;
                next.next();
                Ok(())
            }

            _ => {
                next.next();
                Ok(())
            }
        }
    }

    fn deleted(&mut self, message: &ReconciliationMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            ReconciliationMsg::ReloadState(
                reconciliation_data_body,
                reconciliation_data_metadata,
            ) => {
                self.set_state(reconciliation_data_body, reconciliation_data_metadata, true)?;
                next.next();
                Ok(())
            }
            _ => {
                next.next();
                Ok(())
            }
        }
    }

    fn updated(&mut self, message: &ReconciliationMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            ReconciliationMsg::ReloadState(
                reconciliation_data_body,
                reconciliation_data_metadata,
            ) => {
                self.set_state(
                    reconciliation_data_body,
                    reconciliation_data_metadata,
                    false,
                )?;
                next.next();
                Ok(())
            }
            _ => {
                next.next();
                Ok(())
            }
        }
    }

    fn custom(
        &mut self,
        message: &ReconciliationMsg,
        _next: Option<&StreamNext>,
    ) -> Result<(), AnyError> {
        match message {
            ReconciliationMsg::SelfSender(reconciliation_processor_sender) => {
                self.reconciliation_processor_sender =
                    Some(reconciliation_processor_sender.clone());
                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn event(&mut self, consumer_event: ConsumerEvent) -> Result<(), AnyError> {
        match consumer_event {
            ConsumerEvent::Terminating(name) => {
                if name == "ReconciliationDataConsumer".to_owned() {
                    block_on(self.status_processor_sender.send(ProcessorMessage::Input(
                        InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::State(
                                StatusState::Ready,
                                self.status_processor_sender.clone(),
                            ),
                            None,
                        ),
                    )))?;
                    self.spawn_command_consumers()?;
                } else {
                    info!("Terminating {}", name);
                }
            }
            event => info!("Processor received consumer event: {:?}", event),
        }
        Ok(())
    }
}

impl ReconciliationProcessor {
    fn set_state(
        &mut self,
        reconciliation_data_body: &ReconciliationDataBody,
        reconciliation_data_metadata: &ReconciliationDataMetadata,
        is_delete: bool,
    ) -> Result<(), AnyError> {
        info!("Set state");
        block_on(
            self.status_processor_sender
                .send(ProcessorMessage::Input(InputMessage::new(
                    InputEvent::Custom,
                    StatusMsg::State(StatusState::Loading, self.status_processor_sender.clone()),
                    None,
                ))),
        )?;
        match reconciliation_data_body {
            ReconciliationData::Topics(ref topics) => {
                self.setup_topics_and_spawn_reconciliation_producer(topics)?
            }
            _ => {
                if is_delete {
                    self.reconciliation
                        .set_state_delete(reconciliation_data_body, reconciliation_data_metadata)?;
                } else {
                    self.reconciliation
                        .set_state(reconciliation_data_body, reconciliation_data_metadata)?;
                }
            }
        }

        Ok(())
    }

    fn setup_topics_and_spawn_reconciliation_producer(
        &mut self,
        topics: &ReconciliationDataTopics,
    ) -> Result<(), AnyError> {
        info!("Setup topics");
        self.anonymizations_topics = topics.anonymizations_topics.clone();
        self.raw_records_topics = topics.raw_records_topics.clone();

        info!(
            "Start dynamic producer for {:?}",
            topics.reconciliations_records_topic
        );
        let record_producer_sender = Producer::spawn(
            RecordsProducer::new(topics.reconciliations_records_topic.clone()),
            Some(100),
        )?;
        self.record_producer_sender = Some(record_producer_sender);

        Ok(())
    }

    fn spawn_command_consumers(&mut self) -> Result<(), AnyError> {
        info!("Spawn command consumers");
        if self.reconciliation.is_ready() {
            self.spawn_records_consumers()?;
        } else {
            if let Some(ref reconciliation_processor_sender) = self.reconciliation_processor_sender
            {
                info!("Start dynamic Command consumer");
                Consumer::spawn(
                    ReconciliationProcessConsumer::new(),
                    ProcessorThread::Spawned(reconciliation_processor_sender.clone()),
                )
                .map_err(|source| ResourcesError::ReconciliationProcessConsumerSpawn { source })?;
            } else {
                bail!(ResourcesError::ReconciliationMissingSelfProcessorSender);
            }
        }

        Ok(())
    }

    fn spawn_records_consumers(&mut self) -> Result<(), AnyError> {
        info!("Spawn records consumers");
        if self.reconciliation.is_ready() {
            if let Some(ref reconciliation_processor_sender) = self.reconciliation_processor_sender
            {
                block_on(async {
                    let status_mapping_id = self.status_processor_sender.send(
                        ProcessorMessage::Input(InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::MappingId(
                                self.reconciliation.mapping_id()?,
                                self.status_processor_sender.clone(),
                            ),
                            None,
                        )),
                    );

                    let (profiles_count, reconciliations_count) = self.reconciliation.counts();
                    let status_counts = self.status_processor_sender.send(ProcessorMessage::Input(
                        InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::Counts(profiles_count, reconciliations_count),
                            None,
                        ),
                    ));

                    try_join(status_mapping_id, status_counts).await?;

                    Ok::<(), AnyError>(())
                })?;

                info!(
                    "Start dynamic consumer for {:#?}",
                    self.anonymizations_topics
                );
                Consumer::spawn(
                    AnonymizationsConsumer::new(self.anonymizations_topics.clone()),
                    ProcessorThread::Spawned(reconciliation_processor_sender.clone()),
                )
                .map_err(|source| {
                    ResourcesError::ReconciliationAnonymizationsConsumerSpawn { source }
                })?;

                info!("Start dynamic consumer for {:#?}", self.raw_records_topics);
                Consumer::spawn(
                    RawRecordsConsumer::new(self.raw_records_topics.clone()),
                    ProcessorThread::Spawned(reconciliation_processor_sender.clone()),
                )
                .map_err(|source| {
                    ResourcesError::ReconciliationRawRecordsConsumerSpawn { source }
                })?;

                info!("Processing Records...");
                block_on(self.status_processor_sender.send(ProcessorMessage::Input(
                    InputMessage::new(
                        InputEvent::Custom,
                        StatusMsg::Reconciliations(StatusReconciliation::ProcessingRecords),
                        None,
                    ),
                )))?;
            } else {
                bail!(ResourcesError::ReconciliationMissingSelfProcessorSender);
            }
        }
        Ok(())
    }
}
