use super::StatusMsg;
use crate::producers::StatusProducerSender;
use crate::resources::{Status, StatusImportation};
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::processor::StreamProcessor;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::processor::{ProcessorMessage, ProcessorSender};
use data_stream::producer::ProducerMessage;
use data_stream::stream::{InputEvent, InputMessage, OutputMessage, StreamNext};
use derive_new::new;
use log::{error, info, warn};
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
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
            StatusMsg::Importing(import_instance_id, status_processor_sender) => {
                self.status.import_instance_id = Some(import_instance_id.clone());
                self.status.importation = StatusImportation::ImportingFiles;
                self.tick(status_processor_sender.clone());
                Ok(())
            }
            StatusMsg::Progression(record_import_instance_progression) => {
                if record_import_instance_progression.is_finished {
                    self.status.importation = StatusImportation::ImportationCompleted;
                    info!("Importation completed");
                    Task::spawn(async {
                        Timer::new(Duration::from_secs(30)).await;
                        warn!("Timeout reached... auto-exiting...");

                        if let Err(e) = signal::kill(Pid::this(), Signal::SIGTERM) {
                            error!("Unable to send SIGTERM to myself to auto-exit: {:?}", e);
                        }
                    })
                    .detach();
                }
                self.status.progression = record_import_instance_progression.progression.clone();
                Ok(())
            }
            StatusMsg::Stream => block_on(self.producer.send(ProducerMessage::Output(
                OutputMessage::new(self.status.clone(), None),
            )))
            .map_err(|e| anyhow!(e)),
        }
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
