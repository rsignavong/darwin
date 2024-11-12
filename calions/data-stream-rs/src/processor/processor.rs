use super::{ProcessorConfig, ProcessorConfigBuilder, ProcessorMessage, ProcessorSender};
use super::{ProcessorConfigError, ProcessorCreatedError, ProcessorCustomError};
use super::{ProcessorDeletedError, ProcessorReadError, ProcessorUpdatedError};
use super::{ProcessorError, ProcessorEventError};
use crate::consumer::ConsumerEvent;
use crate::stream::{InputEvent, InputMessage, InputStream};
use crate::stream::{StreamEvent, StreamNext};
use async_channel::{bounded, unbounded};
use async_executor::{Executor, LocalExecutor};
use cfg_if::cfg_if;
use derivative::Derivative;
use easy_parallel::Parallel;
use log::{debug, error, info};
use parking_lot::Mutex;
use rayon::ThreadPoolBuilder;
use smol::future::pending;
use std::fmt::Debug;
use std::sync::Arc;

pub trait StreamProcessor<IS: Debug>: Send + Sync {
    fn config(
        &self,
        builder: &mut ProcessorConfigBuilder,
    ) -> Result<ProcessorConfig, ProcessorConfigError>;

    fn created(&mut self, message: &IS, next: &StreamNext) -> Result<(), ProcessorCreatedError>;

    fn deleted(&mut self, message: &IS, next: &StreamNext) -> Result<(), ProcessorDeletedError>;

    fn updated(&mut self, message: &IS, next: &StreamNext) -> Result<(), ProcessorUpdatedError>;

    fn read(&mut self, message: &IS, next: &StreamNext) -> Result<(), ProcessorReadError> {
        info!("Processor received read message: {:?}", message);
        next.next();
        Ok(())
    }

    fn custom(
        &mut self,
        message: &IS,
        next: Option<&StreamNext>,
    ) -> Result<(), ProcessorCustomError>;

    fn event(&mut self, event: ConsumerEvent) -> Result<(), ProcessorEventError> {
        info!("Processor received consumer event: {:?}", event);
        Ok(())
    }
}

#[derive(Clone)]
pub enum ProcessorThread<IS>
where
    IS: InputStream,
{
    Main(Processor<IS>),
    Spawned(ProcessorSender<IS>),
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Processor<IS> {
    config: ProcessorConfig,
    #[derivative(Debug = "ignore")]
    stream_processor: Arc<Mutex<Box<dyn StreamProcessor<IS>>>>,
}

impl<IS> Processor<IS>
where
    IS: InputStream + Clone + Debug + Send + Sync + 'static,
{
    pub(crate) fn dispatch(&self, input_message: InputMessage<IS>) -> Result<(), ProcessorError> {
        let event = input_message.event();
        let message = input_message.message();
        debug!("{}: Dispatch\n{:?}\n{:#?}", self.name(), event, message);
        let mut stream_processor = self.stream_processor.lock();
        match event {
            InputEvent::Stream(stream_event) => {
                let next = input_message
                    .next()
                    .ok_or_else(|| ProcessorError::ProcessorStreamNext)?;
                match stream_event {
                    StreamEvent::Created => stream_processor.created(message, &next)?,
                    StreamEvent::Deleted => stream_processor.deleted(message, &next)?,
                    StreamEvent::Updated => stream_processor.updated(message, &next)?,
                    StreamEvent::Read => stream_processor.read(message, &next)?,
                }
            }
            InputEvent::Custom => stream_processor.custom(message, input_message.next())?,
        }

        Ok(())
    }

    pub fn name(&self) -> &'static str {
        self.config.name
    }

    pub(crate) fn notify(&self, consumer_event: ConsumerEvent) -> Result<(), ProcessorError> {
        debug!("{}: Notify\n{:?}", self.name(), consumer_event);
        let mut stream_processor = self.stream_processor.lock();
        Ok(stream_processor.event(consumer_event)?)
    }

    pub fn spawn<SP>(
        stream_processor: SP,
        bound: Option<usize>,
        nb_processors: usize,
    ) -> Result<ProcessorSender<IS>, ProcessorError>
    where
        SP: StreamProcessor<IS> + 'static,
    {
        if nb_processors == 0 {
            return Err(ProcessorError::ProcessorSpawnZeroThread);
        }

        let (sender, receiver) = if let Some(max) = bound {
            bounded(max)
        } else {
            unbounded()
        };
        let processor = Self::new(stream_processor)?;
        info!(
            "{}: Starting {} processor(s) thread",
            processor.name(),
            nb_processors
        );

        let pool = ThreadPoolBuilder::new().num_threads(1).build()?;
        pool.spawn(move || {
            let ex = Executor::new();
            let local_ex = LocalExecutor::new();

            cfg_if! {
                if #[cfg(not(feature = "tokio-processors"))] {
                    Parallel::new()
                        .each(0..nb_processors, |_| {
                            let task = ex.spawn(async move {
                                    while let Ok(msg) = receiver.recv().await {
                                        match msg {
                                            ProcessorMessage::Drop => break,
                                            ProcessorMessage::Event(consumer_event) => {
                                                if let Err(e) = processor.notify(consumer_event) {
                                                    error!("{}: {:?}", processor.name(), e);
                                                };
                                            }
                                            ProcessorMessage::Input(input_message) => {
                                                if let Err(e) = processor.dispatch(input_message) {
                                                    error!("{}: {:?}", processor.name(), e);
                                                };
                                            }
                                        }
                                    }
                                    drop(receiver);
                                    drop(processor);
                                });
                            ex.run(task)
                        })
                        .finish(|| ex.enter(|| local_ex.run(pending::<()>())));
                } else {
                    let (trigger, shutdown) = unbounded::<()>();
                    match tokio::runtime::Builder::new()
                        .enable_all()
                        .threaded_scheduler()
                        .core_threads(nb_processors)
                        .build() {
                        Err(e) => error!("{}: Cannot start Tokio runtime: {:?}", processor.name(), e),
                        Ok(mut rt) => {
                            let handle = rt.handle().clone();
                            Parallel::new()
                                .add(|| ex.enter(|| rt.block_on(shutdown.recv())))
                                .each(0..nb_processors, |_| {
                                    handle.enter(|| {
                                        let task = ex.spawn(async move {
                                            while let Ok(msg) = receiver.recv().await {
                                                match msg {
                                                    ProcessorMessage::Drop => break,
                                                    ProcessorMessage::Event(consumer_event) => {
                                                        if let Err(e) = processor.notify(consumer_event) {
                                                            error!("{}: {:?}", processor.name(), e);
                                                        };
                                                    }
                                                    ProcessorMessage::Input(input_message) => {
                                                        if let Err(e) = processor.dispatch(input_message) {
                                                            error!("{}: {:?}", processor.name(), e);
                                                        };
                                                    }
                                                }
                                            }
                                            drop(receiver);
                                            drop(processor);
                                            Ok(())
                                        });
                                        ex.run(task)
                                    })
                                })
                                .finish(|| handle.enter(|| ex.enter(|| local_ex.run(async move {
                                    let _trigger = trigger;
                                    pending::<()>().await
                                }))));
                        }
                    }
                }
            }
        });

        Ok(sender)
    }

    pub fn new<SP>(stream_processor: SP) -> Result<Self, ProcessorError>
    where
        SP: StreamProcessor<IS> + 'static,
    {
        let mut builder = ProcessorConfigBuilder::default();
        Ok(Processor {
            config: stream_processor.config(&mut builder)?,
            stream_processor: Arc::new(Mutex::new(Box::new(stream_processor))),
        })
    }
}
