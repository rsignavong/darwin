use super::ImportationMsg;
use crate::processors::{StatusMsg, StatusProcessorSender};
use crate::producers::{ImportationsProducer, ImportationsProducerSender};
use crate::resources::{CsvLineCount, Importation, ImportationFilesProgressions};
use crate::resources::{ProcessorId, S3AccessKey, S3SecretAccessKey};
use anyhow::{anyhow, Error as AnyError};
use data_stream::processor::ProcessorMessage;
use data_stream::processor::StreamProcessor;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::producer::{Producer, ProducerMessage};
use data_stream::stream::{InputEvent, InputMessage, StreamNext};
use derive_new::new;
use futures::future::try_join_all;
use log::{info, warn};
use rusoto_core::{request::HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::S3Client;
use smol::{block_on, Task};
use std::sync::{atomic::AtomicU64, Arc};

const NAME: &'static str = "ImportationProcessor";

#[derive(new)]
pub struct ImportationProcessor {
    access_key: S3AccessKey,
    secret_access_key: S3SecretAccessKey,
    processor_id: Arc<ProcessorId>,
    status_processor_sender: StatusProcessorSender,
}

impl StreamProcessor<ImportationMsg> for ImportationProcessor {
    fn config(&self, builder: &mut ProcessorConfigBuilder) -> Result<ProcessorConfig, AnyError> {
        builder.name(NAME).build().map_err(|e| anyhow!(e))
    }

    fn created(&mut self, message: &ImportationMsg, next: &StreamNext) -> Result<(), AnyError> {
        match message {
            ImportationMsg::ImportInstance(instance) => {
                if !self.processor_id.eq(&instance.processor_id) {
                    warn!(
                        "Created message skipped: wanted {}, received {}",
                        self.processor_id, instance.processor_id
                    );
                    next.next();
                    return Ok(());
                }

                info!("Import instance id: {}", instance.id);
                block_on(async {
                    self.status_processor_sender
                        .send(ProcessorMessage::Input(InputMessage::new(
                            InputEvent::Custom,
                            StatusMsg::Importing(
                                instance.id.clone(),
                                self.status_processor_sender.clone(),
                            ),
                            None,
                        )))
                        .await?;

                    info!(
                        "Start dynamic producer for {}",
                        instance.config.producer_topic
                    );
                    let importation_producer_sender: ImportationsProducerSender = Producer::spawn(
                        ImportationsProducer::new(instance.config.producer_topic.to_owned()),
                        None,
                    )?;
                    let total_lines = Arc::new(CsvLineCount::from(instance));
                    let files_progressions: Arc<ImportationFilesProgressions> =
                        Arc::new(AtomicU64::new(0));
                    let s3_client = S3Client::new_with(
                        HttpClient::new()?,
                        StaticProvider::new_minimal(
                            self.access_key.to_string(),
                            self.secret_access_key.to_string(),
                        ),
                        Region::Custom {
                            name: instance
                                .config
                                .storage
                                .region_name
                                .as_ref()
                                .map(|region| region.to_string())
                                .unwrap_or_default(),
                            endpoint: instance.config.storage.endpoint.to_string(),
                        },
                    );

                    info!("{} file(s) to import", instance.config.files.len());

                    let mut tasks: Vec<_> = Vec::new();

                    for file in &*instance.config.files {
                        let bucket = instance.config.storage.bucket.clone();
                        let file = file.clone();
                        let files_progressions = files_progressions.clone();
                        let import_instane_id = instance.id.clone();
                        let import = instance.config.import.clone();
                        let mapping = instance.config.mapping.clone();
                        let mapping_id = instance.config.activated_mapping_id.clone();
                        let producer = importation_producer_sender.clone();
                        let s3_client = s3_client.clone();
                        let status_processor_sender = self.status_processor_sender.clone();
                        let total_lines = total_lines.clone();

                        tasks.push(Task::spawn(async move {
                            let importation = Importation::new(
                                bucket,
                                file,
                                files_progressions,
                                import,
                                import_instane_id,
                                mapping,
                                mapping_id,
                                producer,
                                s3_client,
                                status_processor_sender,
                                total_lines,
                            );

                            importation.import().await
                        }));
                    }
                    try_join_all(tasks).await?;
                    importation_producer_sender
                        .send(ProducerMessage::Drop)
                        .await?;
                    info!("All files read");
                    Ok::<(), AnyError>(())
                })?;
                next.next();
                Ok(())
            }
        }
    }

    fn deleted(&mut self, _message: &ImportationMsg, next: &StreamNext) -> Result<(), AnyError> {
        warn!("Updated message: skip");
        next.next();
        Ok(())
    }

    fn updated(&mut self, _message: &ImportationMsg, next: &StreamNext) -> Result<(), AnyError> {
        next.next();
        Ok(())
    }

    fn custom(
        &mut self,
        _message: &ImportationMsg,
        _next: Option<&StreamNext>,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}
