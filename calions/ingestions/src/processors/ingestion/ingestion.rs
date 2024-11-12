use super::msg::IngestionMsg;
use crate::consumers::ActivatedMappingsConsumer;
use crate::consumers::{AnonymizationsConsumer, ReconciliationRecordsConsumer};
use crate::decoders::{IngestionDataBody, IngestionDataMetadata};
use crate::processors::{StatusMsg, StatusProcessorSender};
use crate::producers::IngestionDataProducerSender;
use crate::producers::{ContactsProducerSender, GdprKeysProducerSender};
use crate::resources::StatusState;
use crate::resources::{ContactError, GdprError, IngestionError};
use crate::resources::{ContactType, GdprKeyType, Ingestion, StatusIngestion};
use anyhow::{anyhow, bail, ensure, Error as AnyError};
use data_stream::consumer::{Consumer, ConsumerEvent};
use data_stream::processor::StreamProcessor;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::processor::{ProcessorMessage, ProcessorSender, ProcessorThread};
use data_stream::producer::ProducerMessage;
use data_stream::stream::{InputEvent, InputMessage, OutputMessage, StreamNext};
use derive_new::new;
use futures::future::try_join;
use log::info;
use smol::block_on;

const NAME: &'static str = "IngestionProcessor";

pub type IngestionProcessorSender = ProcessorSender<IngestionMsg>;

#[derive(new)]
pub struct IngestionProcessor {
    contacts_producer_sender: ContactsProducerSender,
    gdpr_keys_producer_sender: GdprKeysProducerSender,
    ingestion: Ingestion,
    ingestion_data_producer_sender: IngestionDataProducerSender,
    #[new(default)]
    ingestion_processor_sender: Option<IngestionProcessorSender>,
    status_processor_sender: StatusProcessorSender,
}

impl StreamProcessor<IngestionMsg> for IngestionProcessor {
    fn config(&self, builder: &mut ProcessorConfigBuilder) -> Result<ProcessorConfig, AnyError> {
        builder.name(NAME).build().map_err(|e| anyhow!(e))
    }

    fn created(&mut self, message: &IngestionMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            IngestionMsg::Anonymize(anonymization_body) => block_on(async {
                if let Some(key) = self.ingestion.anonymize_contact(anonymization_body).await? {
                    ensure!(
                        key.type_ == GdprKeyType::Deleted,
                        GdprError::GdprKeyNotDeleted
                    );

                    self.gdpr_keys_producer_sender
                        .send(ProducerMessage::Output(OutputMessage::new(
                            key,
                            Some(next.clone()),
                        )))
                        .await?;
                } else {
                    next.next();
                }

                Ok::<(), AnyError>(())
            }),

            IngestionMsg::Ingest(record_body, record_metadata) => block_on(async {
                if let Some((contact, key)) =
                    self.ingestion.create(record_body, record_metadata).await?
                {
                    ensure!(
                        contact.type_ == ContactType::Created,
                        ContactError::ContactNotCreated
                    );

                    ensure!(
                        key.type_ == GdprKeyType::Created,
                        GdprError::GdprKeyNotCreated
                    );

                    self.status_processor_sender
                        .send(ProcessorMessage::Input(InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::Count(contact.count.clone()),
                            None,
                        )))
                        .await?;
                    self.gdpr_keys_producer_sender
                        .send(ProducerMessage::Output(OutputMessage::new(key, None)))
                        .await?;
                    self.contacts_producer_sender
                        .send(ProducerMessage::Output(OutputMessage::new(
                            contact,
                            Some(next.clone()),
                        )))
                        .await?;
                } else {
                    next.next();
                }
                Ok::<(), AnyError>(())
            }),

            IngestionMsg::Mapping(mapping_body) => {
                if let Some(ingestion_data_record) = self.ingestion.set_mapping(mapping_body)? {
                    block_on(
                        self.ingestion_data_producer_sender
                            .send(ProducerMessage::Output(OutputMessage::new(
                                ingestion_data_record,
                                None,
                            ))),
                    )?;
                }
                next.next();
                self.spawn_records_consumers()?;
                Ok(())
            }

            IngestionMsg::ReloadState(ingestion_data_body, ingestion_data_metadata) => {
                self.set_state(ingestion_data_body, ingestion_data_metadata)?;
                next.next();
                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn deleted(&mut self, _message: &IngestionMsg, next: &StreamNext) -> Result<(), AnyError> {
        next.next();
        Ok(())
    }

    fn updated(&mut self, message: &IngestionMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            IngestionMsg::Ingest(record_body, record_metadata) => block_on(async {
                if let Some((contact, contacts)) =
                    self.ingestion.update(record_body, record_metadata).await?
                {
                    ensure!(
                        contact.type_ == ContactType::Updated,
                        ContactError::ContactNotUpdated
                    );

                    for (contact, key) in contacts {
                        ensure!(
                            contact.type_ == ContactType::Deleted,
                            ContactError::ContactNotDeleted
                        );
                        ensure!(
                            key.type_ == GdprKeyType::Deleted,
                            GdprError::GdprKeyNotDeleted
                        );

                        self.status_processor_sender
                            .send(ProcessorMessage::Input(InputMessage::new(
                                InputEvent::Custom,
                                StatusMsg::Count(contact.count.clone()),
                                None,
                            )))
                            .await?;
                        self.gdpr_keys_producer_sender
                            .send(ProducerMessage::Output(OutputMessage::new(key, None)))
                            .await?;
                        self.contacts_producer_sender
                            .send(ProducerMessage::Output(OutputMessage::new(contact, None)))
                            .await?;
                    }

                    self.status_processor_sender
                        .send(ProcessorMessage::Input(InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::Count(contact.count.clone()),
                            None,
                        )))
                        .await?;
                    self.contacts_producer_sender
                        .send(ProducerMessage::Output(OutputMessage::new(
                            contact,
                            Some(next.clone()),
                        )))
                        .await?;
                } else {
                    next.next();
                }

                Ok::<(), AnyError>(())
            }),

            IngestionMsg::ReloadState(ingestion_data_body, ingestion_data_metadata) => {
                self.set_state(ingestion_data_body, ingestion_data_metadata)?;
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
        message: &IngestionMsg,
        _next: Option<&StreamNext>,
    ) -> Result<(), AnyError> {
        match message {
            IngestionMsg::SelfSender(ingestion_processor_sender) => {
                self.ingestion_processor_sender = Some(ingestion_processor_sender.clone());
                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn event(&mut self, consumer_event: ConsumerEvent) -> Result<(), AnyError> {
        match consumer_event {
            ConsumerEvent::Terminating(name) => {
                if name == "IngestionDataConsumer".to_owned() {
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
                    self.spawn_mappings_consumers()?;
                } else {
                    info!("Terminating {}", name);
                }
            }
            event => info!("Processor received consumer event: {:?}", event),
        }
        Ok(())
    }
}

impl IngestionProcessor {
    fn set_state(
        &mut self,
        ingestion_data_body: &IngestionDataBody,
        ingestion_data_metadata: &IngestionDataMetadata,
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
        self.ingestion
            .set_state(ingestion_data_body, ingestion_data_metadata)?;

        Ok(())
    }

    fn spawn_mappings_consumers(&mut self) -> Result<(), AnyError> {
        info!("Spawn mappings consumers");
        if self.ingestion.is_ready() {
            self.spawn_records_consumers()?;
        } else {
            if let Some(ref ingestion_processor_sender) = self.ingestion_processor_sender {
                info!("Start dynamic Mappings consumer");

                Consumer::spawn(
                    ActivatedMappingsConsumer::new(),
                    ProcessorThread::Spawned(ingestion_processor_sender.clone()),
                )
                .map_err(|source| IngestionError::ActivatedMappingsConsumerSpawn { source })?;
            } else {
                bail!(IngestionError::IngestionMissingSelfProcessorSender);
            }
        }

        Ok(())
    }
    fn spawn_records_consumers(&mut self) -> Result<(), AnyError> {
        if self.ingestion.is_ready() {
            if let Some(ref ingestion_processor_sender) = self.ingestion_processor_sender {
                block_on(async {
                    let status_mapping_id = self.status_processor_sender.send(
                        ProcessorMessage::Input(InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::MappingId(
                                self.ingestion.mapping_id()?,
                                self.status_processor_sender.clone(),
                            ),
                            None,
                        )),
                    );
                    let status_count = self.status_processor_sender.send(ProcessorMessage::Input(
                        InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::Count(self.ingestion.count().clone()),
                            None,
                        ),
                    ));

                    try_join(status_mapping_id, status_count).await?;

                    Ok::<(), AnyError>(())
                })?;

                Consumer::spawn(
                    AnonymizationsConsumer::new(),
                    ProcessorThread::Spawned(ingestion_processor_sender.clone()),
                )
                .map_err(|source| {
                    IngestionError::IngestionAnonymizationsConsumerSpawn { source }
                })?;

                Consumer::spawn(
                    ReconciliationRecordsConsumer::new(),
                    ProcessorThread::Spawned(ingestion_processor_sender.clone()),
                )
                .map_err(|source| {
                    IngestionError::IngestionReconciliationRecordsConsumerSpawn { source }
                })?;

                info!("Processing Records...");
                block_on(self.status_processor_sender.send(ProcessorMessage::Input(
                    InputMessage::new(
                        InputEvent::Custom,
                        StatusMsg::Ingestions(StatusIngestion::ProcessingContacts),
                        None,
                    ),
                )))?;
            } else {
                bail!(IngestionError::IngestionMissingSelfProcessorSender);
            }
        }
        Ok(())
    }
}
