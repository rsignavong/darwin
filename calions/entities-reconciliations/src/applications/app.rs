use crate::consumers::ReconciliationCommandConsumer;
use crate::processors::{ReconciliationMsg, ReconciliationProcessor, StatusMsg, StatusProcessor};
use crate::producers::{StatusProducer, StatusProducerSender};
use crate::resources::{Reconciliation, Status};
use crate::Settings;
use anyhow::anyhow;
use data_stream::consumer::Consumer;
use data_stream::error::DataStreamError;
use data_stream::processor::{Processor, ProcessorMessage, ProcessorThread};
use data_stream::producer::Producer;
use data_stream::stream::{InputEvent, InputMessage};
use log::info;

pub struct App;

impl App {
    pub async fn start() -> Result<(), DataStreamError> {
        info!("Starting");
        let oid = Settings::get().organization_id.clone();
        let pid = Settings::get().processor_id.clone();

        let status = Status::new(pid.clone());
        let status_producer_sender: StatusProducerSender =
            Producer::spawn(StatusProducer::new(), Some(5))?;
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

        let reconciliation = Reconciliation::new(pid.clone());
        let reconciliation_processor_sender = Processor::spawn(
            ReconciliationProcessor::new(reconciliation, status_processor_sender),
            Some(100),
            8,
        )?;
        reconciliation_processor_sender
            .send(ProcessorMessage::Input(InputMessage::new(
                InputEvent::Custom,
                ReconciliationMsg::SelfSender(reconciliation_processor_sender.clone()),
                None,
            )))
            .await
            .map_err(|e| anyhow!(e))?;

        Consumer::spawn(
            ReconciliationCommandConsumer::new(),
            ProcessorThread::Spawned(reconciliation_processor_sender),
        )?;

        info!("Processor id: {}", pid);
        info!("Organization id: {}", oid);

        Ok(())
    }
}
