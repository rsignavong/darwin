use super::msg::ReconciliationMsg;
use crate::consumers::EntityRecordsConsumer;
use crate::decoders::{Entity, EntityRecordBody, EntityRecordMetadata};
use crate::processors::{StatusMsg, StatusProcessorSender};
use crate::producers::{ReconciliationsProducer, ReconciliationsProducerSender};
use crate::resources::ResourcesError;
use crate::resources::{EntityType, Reconciliation, StatusReconciliation, StatusState};
use anyhow::{anyhow, bail, Error as AnyError};
use data_stream::consumer::Consumer;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder, StreamProcessor};
use data_stream::processor::{ProcessorMessage, ProcessorSender, ProcessorThread};
use data_stream::producer::{Producer, ProducerMessage};
use data_stream::stream::{InputEvent, InputMessage, OutputMessage, StreamNext};
use derive_new::new;
use log::info;
use smol::block_on;
use std::sync::Arc;

const NAME: &'static str = "ReconciliationProcessor";

pub type ReconciliationProcessorSender = ProcessorSender<ReconciliationMsg>;

#[derive(new)]
pub struct ReconciliationProcessor {
    reconciliation: Reconciliation,
    #[new(default)]
    reconciliation_processor_sender: Option<ReconciliationProcessorSender>,
    #[new(default)]
    reconciliations_producer_sender: Option<ReconciliationsProducerSender>,
    status_processor_sender: StatusProcessorSender,
}

impl StreamProcessor<ReconciliationMsg> for ReconciliationProcessor {
    fn config(&self, builder: &mut ProcessorConfigBuilder) -> Result<ProcessorConfig, AnyError> {
        builder.name(NAME).build().map_err(|e| anyhow!(e))
    }

    fn created(&mut self, message: &ReconciliationMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            ReconciliationMsg::Configure(cmd) => {
                if let Some(mapping_id) = self.reconciliation.configure(cmd) {
                    info!("Start dynamic producer for {:?}", cmd.reconciliations.topic);
                    block_on(async {
                        let reconciliations_producer_sender = Producer::spawn(
                            ReconciliationsProducer::new(cmd.reconciliations.topic.clone()),
                            Some(100),
                        )?;
                        self.reconciliations_producer_sender =
                            Some(reconciliations_producer_sender);

                        self.status_processor_sender
                            .send(ProcessorMessage::Input(InputMessage::new(
                                InputEvent::Custom,
                                StatusMsg::MappingId(
                                    mapping_id,
                                    self.status_processor_sender.clone(),
                                ),
                                None,
                            )))
                            .await?;

                        // Temporary set state, to remove when reconciliation data is ready

                        self.status_processor_sender
                            .send(ProcessorMessage::Input(InputMessage::new(
                                InputEvent::Custom,
                                StatusMsg::State(
                                    StatusState::Loading,
                                    self.status_processor_sender.clone(),
                                ),
                                None,
                            )))
                            .await?;
                        let status = self.reconciliation.set_temp_state();
                        self.status_processor_sender
                            .send(ProcessorMessage::Input(InputMessage::new(
                                InputEvent::Custom,
                                StatusMsg::State(status, self.status_processor_sender.clone()),
                                None,
                            )))
                            .await?;
                        Ok::<(), AnyError>(())
                    })?;
                }
                next.next();
                self.spawn_entity_records_consumers(&cmd.mappings.entities)?;
                Ok(())
            }

            ReconciliationMsg::Match(entity_type, record_body, record_metadata) => {
                self.match_record(entity_type, record_body, record_metadata, next)
            }

            _ => {
                next.next();
                Ok(())
            }
        }
    }

    fn deleted(&mut self, message: &ReconciliationMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            ReconciliationMsg::Match(entity_type, record_body, record_metadata) => {
                if let Some(records) =
                    self.reconciliation
                        .delete(entity_type, record_body, record_metadata)?
                {
                    if records.is_empty() {
                        next.next();
                        return Ok(());
                    }

                    if let Some(ref reconciliations_producer_sender) =
                        self.reconciliations_producer_sender
                    {
                        block_on(async {
                            let last = records.len() - 1;
                            for (n, record) in records.into_iter().enumerate() {
                                self.status_processor_sender
                                    .send(ProcessorMessage::Input(InputMessage::new(
                                        InputEvent::Custom,
                                        StatusMsg::Counts(record.reconciliations_count.clone()),
                                        None,
                                    )))
                                    .await?;
                                reconciliations_producer_sender
                                    .send(ProducerMessage::Output(OutputMessage::new(
                                        record,
                                        if n == last { Some(next.clone()) } else { None },
                                    )))
                                    .await?;
                            }

                            Ok::<(), AnyError>(())
                        })?;
                    } else {
                        bail!(ResourcesError::ReconciliationMatchRecordOnDeleted);
                    }
                } else {
                    next.next();
                }
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
            ReconciliationMsg::Match(entity_type, record_body, record_metadata) => {
                self.match_record(entity_type, record_body, record_metadata, next)
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
}

impl ReconciliationProcessor {
    fn match_record(
        &mut self,
        entity_type: &Arc<EntityType>,
        record_body: &EntityRecordBody,
        record_metadata: &EntityRecordMetadata,
        next: &StreamNext,
    ) -> Result<(), AnyError> {
        if let Some(records) =
            self.reconciliation
                .match_record(entity_type, record_body, record_metadata)?
        {
            if records.is_empty() {
                next.next();
                return Ok(());
            }

            if let Some(ref reconciliations_producer_sender) = self.reconciliations_producer_sender
            {
                block_on(async {
                    let last = records.len() - 1;
                    for (n, record) in records.into_iter().enumerate() {
                        self.status_processor_sender
                            .send(ProcessorMessage::Input(InputMessage::new(
                                InputEvent::Custom,
                                StatusMsg::Counts(record.reconciliations_count.clone()),
                                None,
                            )))
                            .await?;
                        reconciliations_producer_sender
                            .send(ProducerMessage::Output(OutputMessage::new(
                                record,
                                if n == last { Some(next.clone()) } else { None },
                            )))
                            .await?;
                    }
                    Ok::<(), AnyError>(())
                })?;
            } else {
                bail!(ResourcesError::ReconciliationMatchRecordOnCreatedUpdated);
            }
        } else {
            next.next();
        }
        Ok(())
    }

    fn spawn_entity_records_consumers(&mut self, entities: &[Arc<Entity>]) -> Result<(), AnyError> {
        if self.reconciliation.is_ready() {
            if let Some(ref reconciliation_processor_sender) = self.reconciliation_processor_sender
            {
                for entity in entities.iter() {
                    info!("Start dynamic consumer for {:#?}", entity.topics);
                    Consumer::spawn(
                        EntityRecordsConsumer::new(entity.type_.clone(), entity.topics.clone()),
                        ProcessorThread::Spawned(reconciliation_processor_sender.clone()),
                    )
                    .map_err(|source| {
                        ResourcesError::ReconciliationRawRecordsConsumerSpawn { source }
                    })?;
                }

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
