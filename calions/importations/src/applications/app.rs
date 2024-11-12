use crate::consumers::ImportInstancesConsumer;
use crate::processors::{ImportationProcessor, StatusMsg, StatusProcessor};
use crate::producers::StatusProducer;
use crate::resources::Status;
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
        let pid = Arc::new(Settings::get().processor_id.clone());

        let status = Status::new(pid.clone());
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

        let aws = &Settings::get().aws;
        let importation_processor_sender = Processor::spawn(
            ImportationProcessor::new(
                aws.access_key.clone(),
                aws.secret_access_key.clone(),
                pid.clone(),
                status_processor_sender,
            ),
            Some(100),
            8,
        )?;

        Consumer::spawn(
            ImportInstancesConsumer::new(),
            ProcessorThread::Spawned(importation_processor_sender.clone()),
        )?;

        info!("Processor id: {}", pid);

        Ok(())
    }
}
