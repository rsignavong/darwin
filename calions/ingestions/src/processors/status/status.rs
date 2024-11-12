use super::StatusMsg;
use crate::producers::StatusProducerSender;
use crate::resources::Status;
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::processor::StreamProcessor;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::processor::{ProcessorMessage, ProcessorSender};
use data_stream::producer::ProducerMessage;
use data_stream::stream::{InputEvent, InputMessage, OutputMessage, StreamNext};
use derive_new::new;
use log::{error, info};
use smol::{block_on, Task, Timer};
use std::time::Duration;

const NAME: &'static str = "StatusProcessor";

pub type StatusProcessorSender = ProcessorSender<StatusMsg>;

#[derive(new)]
pub struct StatusProcessor {
    status: Status,
    producer: StatusProducerSender,
    #[new(default)]
    ticking: bool,
}

impl StreamProcessor<StatusMsg> for StatusProcessor {
    fn config(&self, builder: &mut ProcessorConfigBuilder) -> Result<ProcessorConfig, AnyError> {
        builder.name(NAME).build().map_err(|e| anyhow!(e))
    }

    fn created(&mut self, _message: &StatusMsg, _next: &StreamNext) -> Result<(), AnyError> {
        Ok(())
    }

    fn deleted(&mut self, _message: &StatusMsg, _next: &StreamNext) -> Result<(), AnyError> {
        Ok(())
    }

    fn updated(&mut self, _message: &StatusMsg, _next: &StreamNext) -> Result<(), AnyError> {
        Ok(())
    }

    fn custom(&mut self, message: &StatusMsg, _next: Option<&StreamNext>) -> Result<(), AnyError> {
        match message {
            StatusMsg::Count(contacts_count) => {
                self.status.contacts_count = contacts_count.clone();
            }
            StatusMsg::MappingId(mapping_id, status_processor_sender) => {
                self.status.mapping_id = Some(mapping_id.clone());
                self.tick(status_processor_sender.clone());
            }
            StatusMsg::Ingestions(status_ingestion) => {
                self.status.ingestions = status_ingestion.clone();
            }
            StatusMsg::State(status_state, status_processor_sender) => {
                self.status.state = status_state.clone();
                self.tick(status_processor_sender.clone());
            }
            StatusMsg::Stream => {
                block_on(
                    self.producer
                        .send(ProducerMessage::Output(OutputMessage::new(
                            self.status.clone(),
                            None,
                        ))),
                )
                .map_err(|e| anyhow!(e))?;
            }
        };

        Ok(())
    }
}

impl StatusProcessor {
    pub fn tick(&mut self, status_processor_sender: StatusProcessorSender) {
        if self.ticking {
            return;
        }

        info!("Start updating status");
        self.ticking = true;
        Task::spawn(async move {
            loop {
                Timer::new(Duration::from_millis(
                    Settings::get().status_heartbeat_interval,
                ))
                .await;
                if let Err(e) = status_processor_sender
                    .send(ProcessorMessage::Input(InputMessage::new(
                        InputEvent::Custom,
                        StatusMsg::Stream,
                        None,
                    )))
                    .await
                {
                    error!("Status::tick: {:?}", e);
                }
            }
        })
        .detach();
    }
}
