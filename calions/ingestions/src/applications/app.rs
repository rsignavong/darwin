use crate::consumers::IngestionDataConsumer;
use crate::processors::{IngestionMsg, IngestionProcessor, StatusMsg, StatusProcessor};
use crate::producers::{ContactsProducer, GdprKeysProducer, StatusProducer};
use crate::producers::{IngestionDataProducer, IngestionDataProducerSender};
use crate::resources::{Ingestion, Status};
use crate::Settings;
use anyhow::anyhow;
use data_stream::consumer::Consumer;
use data_stream::error::DataStreamError;
use data_stream::processor::{Processor, ProcessorMessage, ProcessorThread};
use data_stream::producer::Producer;
use data_stream::stream::{InputEvent, InputMessage};
use log::info;
use std::sync::Arc;

pub struct App;

impl App {
    pub async fn start() -> Result<(), DataStreamError> {
        info!("Starting");
        let oid = Arc::new(Settings::get().organization_id.clone());
        let pid = Arc::new(Settings::get().processor_id.clone());

        let status = Status::new(oid.clone(), pid.clone());
        let status_producer_sender = Producer::spawn(StatusProducer::new(), Some(5))?;
        let status_processor_sender = Processor::spawn(
            StatusProcessor::new(status.clone(), status_producer_sender),
            Some(5),
            1,
        )?;
        status_processor_sender
            .send(ProcessorMessage::Input(InputMessage::new(
                InputEvent::Custom,
                StatusMsg::Stream,
                None,
            )))
            .await
            .map_err(|e| anyhow!(e))?;

        let ingestion_data_producer_sender: IngestionDataProducerSender =
            Producer::spawn(IngestionDataProducer::new(), Some(100))?;

        let ingestion = Ingestion::new(pid.clone(), oid.clone())
            .await
            .map_err(|e| anyhow!(e))?;

        let contacts_producer_sender = Producer::spawn(ContactsProducer::new(), None)?;
        let gdpr_keys_producer_sender = Producer::spawn(GdprKeysProducer::new(), None)?;
        let ingestion_processor_sender = Processor::spawn(
            IngestionProcessor::new(
                contacts_producer_sender,
                gdpr_keys_producer_sender,
                ingestion,
                ingestion_data_producer_sender,
                status_processor_sender,
            ),
            Some(100),
            8,
        )?;
        ingestion_processor_sender
            .send(ProcessorMessage::Input(InputMessage::new(
                InputEvent::Custom,
                IngestionMsg::SelfSender(ingestion_processor_sender.clone()),
                None,
            )))
            .await
            .map_err(|e| anyhow!(e))?;

        Consumer::spawn(
            IngestionDataConsumer::new(),
            ProcessorThread::Spawned(ingestion_processor_sender),
        )?;

        info!("Processor id: {}", pid);
        info!("Organization id: {}", oid);

        Ok(())
    }
}
