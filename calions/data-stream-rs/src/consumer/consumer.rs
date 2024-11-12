use super::{ConsumerConfig, ConsumerConfigBuilder, ConsumerHalt, ConsumerOffsetStorage};
use super::{ConsumerConfigError, ConsumerError, ConsumerStreamError};
use super::{ConsumerEvent, ConsumerOffsets, ConsumerPool, ConsumerStream};
use crate::common::TopicPartition;
use crate::processor::{ProcessorMessage, ProcessorThread};
use crate::stream::DataStreamOffset;
use crate::stream::{InputEvent, InputMessage, InputStream};
use crate::stream::{Offset as StreamOffset, StreamNext, StreamPayload};
use crate::stream::{Position, ProcessingTime};
use anyhow::anyhow;
use async_channel::unbounded;
use async_executor::{Executor, LocalExecutor};
use easy_parallel::Parallel;
use log::{debug, error, info, warn};
use rdkafka::config::{ClientConfig, FromClientConfig};
use rdkafka::consumer::base_consumer::BaseConsumer;
use rdkafka::consumer::{CommitMode, Consumer as KafkaConsumer};
use rdkafka::error::KafkaError;
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::topic_partition_list::{Offset, TopicPartitionList};
use rdkafka::util::Timeout;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub trait StreamConsumer<SM, SO, E, M, IS>: Send + Sync
where
    SM: Debug,
    SO: Debug,
    E: Debug,
    M: Debug,
    IS: InputStream,
{
    fn config(
        &self,
        builder: &mut ConsumerConfigBuilder,
    ) -> Result<ConsumerConfig, ConsumerConfigError>;

    fn into_input_stream(
        &self,
        stream: StreamPayload<SM, SO, E, M>,
    ) -> Result<IS, ConsumerStreamError>;
}

enum ConsumerLoop {
    Continue,
    Stop,
}

#[derive(Clone)]
pub struct Consumer<SM, SO, E, M, IS>
where
    IS: InputStream,
{
    config: ConsumerConfig,
    kafka_config: ClientConfig,
    processor_thread: ProcessorThread<IS>,
    stream_consumer: Arc<Box<dyn StreamConsumer<SM, SO, E, M, IS>>>,
}

impl<SM, SO, E, M, IS> Consumer<SM, SO, E, M, IS>
where
    SM: Debug + DeserializeOwned + Clone + Send + Sync + 'static,
    SO: Debug + DeserializeOwned + Clone + Send + Sync + 'static,
    E: Debug + DeserializeOwned + Clone + Send + Sync + 'static,
    M: Debug + DeserializeOwned + Clone + Send + Sync + 'static,
    IS: InputStream + Clone + Debug + Send + Sync + 'static,
{
    fn assign_topic_partition(
        &self,
        base_consumer: &BaseConsumer,
        tp: &TopicPartition,
    ) -> Result<(), ConsumerError> {
        info!("{}: Assign {} to kafka consumer", self.name(), tp);
        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(tp.topic(), tp.partition(), Offset::Invalid);

        base_consumer
            .assign(&tpl)
            .map_err(|source| ConsumerError::ConsumerTopicPartitionAssignation { source })?;

        Ok(())
    }

    async fn consume(
        &self,
        consumer: Arc<BaseConsumer>,
        stream: ConsumerStream<SM, SO, E, M>,
        latest_offset: &StreamOffset,
        latest_consumed_offset: Option<&StreamOffset>,
        offset_producer: Option<&BaseProducer>,
    ) -> Result<ConsumerLoop, ConsumerError> {
        let mut next_loop = ConsumerLoop::Continue;

        if let Some(latest_consumed_offset) = latest_consumed_offset {
            if stream.offset <= *latest_consumed_offset {
                warn!("{}: Message already seen\n{}", self.name(), stream);
                return Ok(next_loop);
            }
        }

        if let StreamPayload::DataStreamOffset(data_stream_offset) = &stream.payload {
            let mut offsets = ConsumerOffsets::load().lock();
            offsets.set(
                data_stream_offset.topic_partition().clone(),
                data_stream_offset.offset().clone(),
            );
            return Ok(next_loop);
        }

        let position = stream.offset.position(&latest_offset);
        let event = stream.payload.event();

        let (stream_next, listener) = StreamNext::new();
        let input_message = InputMessage::new(
            InputEvent::Stream(*event),
            self.stream_consumer.into_input_stream(stream.payload)?,
            Some(stream_next),
        );

        debug!("{}: Processing\n{:#?}\n", self.name(), input_message);
        let res = match self.processor_thread {
            ProcessorThread::Main(ref processor) => processor
                .dispatch(input_message)
                .map_err(|source| ConsumerError::ConsumerProcessorDispatch { source }),
            ProcessorThread::Spawned(ref processor_sender) => processor_sender
                .send(ProcessorMessage::Input(input_message))
                .await
                .map_err(|e| ConsumerError::ConsumerProcessorSender { source: anyhow!(e) }),
        };

        if let Err(_) = res {
            error!(
                "{}: Message offset {} not processed",
                self.name(),
                stream.offset,
            );
            return Ok(ConsumerLoop::Continue);
        }

        // replace by oneshot channel in sync wait
        listener.await;

        match &self.config.offset_storage {
            ConsumerOffsetStorage::KafkaProducer(ref topic) => {
                let offset_producer =
                    offset_producer.ok_or_else(|| ConsumerError::ConsumerOffsetProducerMissing)?;
                let key = &stream.topic_partition.to_string();
                let stream_payload =
                    &StreamPayload::<String, String, String, String>::DataStreamOffset(
                        DataStreamOffset::new(
                            self.name().to_owned(),
                            stream.topic_partition,
                            stream.offset.clone(),
                            ProcessingTime::new(),
                        ),
                    );
                let payload = &serde_json::to_string(stream_payload).map_err(|source| {
                    ConsumerError::ConsumerSerializeDataStreamOffset { source }
                })?;

                debug!(
                    "{}: Commit offset to topic: {}\nKey: {}\n{}",
                    self.name(),
                    topic,
                    key,
                    payload
                );

                offset_producer
                    .send::<String, String>(BaseRecord::to(topic).key(key).payload(payload))
                    .map_err(|(source, msg)| ConsumerError::ConsumerSendOffset {
                        source,
                        msg: format!("{:?}", msg),
                    })?;

                if *stream.offset % 1_000 == 0 {
                    info!("{}: Kafka commit", self.name());
                    consumer
                        .commit(&stream.topic_partition_list, CommitMode::Async)
                        .map_err(|source| ConsumerError::ConsumerCommitMessage { source })?;
                }
            }
            ConsumerOffsetStorage::InMemory => (),
        }

        info!(
            "{}: Message offset {} processed",
            self.name(),
            stream.offset
        );

        if self.config.halt == ConsumerHalt::Auto && position == Position::Latest {
            next_loop = ConsumerLoop::Stop;
        }

        Ok(next_loop)
    }

    fn create_base_consumer(&self) -> Result<BaseConsumer, ConsumerError> {
        info!("{}: Create kafka consumer", self.name());
        let consumer = BaseConsumer::from_config(&self.kafka_config)
            .map_err(|source| ConsumerError::ConsumerBaseConsumerCreate { source })?;

        Ok(consumer)
    }

    fn create_offset_producer(&self) -> Result<Option<BaseProducer>, ConsumerError> {
        let config = &self.config;
        let offset_producer = match config.offset_storage {
            ConsumerOffsetStorage::KafkaProducer(_) => {
                let mut kafka_config = ClientConfig::new();
                kafka_config
                    .set("bootstrap.servers", &config.brokers.join(","))
                    .set("message.timeout.ms", &config.timeout.to_string());

                if let Some(debug) = &config.debug {
                    kafka_config.set("debug", &debug);
                }

                if let Some(level) = &config.log_level {
                    kafka_config.set_log_level(*level);
                }

                let producer: BaseProducer = kafka_config
                    .create()
                    .map_err(|source| ConsumerError::ConsumerCreateClientProducer { source })?;

                Some(producer)
            }
            ConsumerOffsetStorage::InMemory => None,
        };

        Ok(offset_producer)
    }

    fn get_latest_offsets(
        &self,
        base_consumer: &BaseConsumer,
    ) -> Result<Vec<(TopicPartition, StreamOffset)>, ConsumerError> {
        let mut offsets = Vec::new();
        let topic_partition_list = Self::topic_partition_list(&base_consumer, &self.config.topics)?;
        for tp in topic_partition_list {
            let (_, end_offset) = base_consumer
                .fetch_watermarks(tp.topic(), tp.partition(), Duration::from_secs(30))
                .map_err(|source| ConsumerError::ConsumerFetchWatermarks { source })?;
            let latest_offset: StreamOffset = (end_offset - 1).into();

            info!(
                "{}: Topic-Partition {} / Latest Offset {}",
                self.config.name, tp, latest_offset
            );
            offsets.push((tp, latest_offset));
        }

        Ok(offsets)
    }

    pub fn name(&self) -> &'static str {
        self.config.name
    }

    fn new<SC>(
        stream_consumer: SC,
        processor_thread: ProcessorThread<IS>,
    ) -> Result<Self, ConsumerError>
    where
        SC: StreamConsumer<SM, SO, E, M, IS> + Send + Sync + 'static,
    {
        let mut builder = ConsumerConfigBuilder::default();
        let config = stream_consumer.config(&mut builder)?;

        match &config.offset_storage {
            ConsumerOffsetStorage::KafkaProducer(topic) => {
                if topic.is_empty() {
                    Err(ConsumerConfigError::ConsumerConfigOffsetStorageTopicEmpty)?;
                } else {
                    info!("{}: Offset storage: {}", config.name, topic);
                }
            }
            ConsumerOffsetStorage::InMemory => info!("{}: Offset storage: In-Memory", config.name),
        }

        let mut kafka_config = ClientConfig::new();
        kafka_config
            .set("group.id", &config.group_id)
            .set("bootstrap.servers", &config.brokers.join(","))
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", &config.timeout.to_string())
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", &config.offset_reset);

        if let Some(debug) = &config.debug {
            kafka_config.set("debug", &debug);
        }

        if let Some(level) = &config.log_level {
            kafka_config.set_log_level(*level);
        }

        Ok(Consumer {
            config,
            kafka_config,
            processor_thread,
            stream_consumer: Arc::new(Box::new(stream_consumer)),
        })
    }

    pub fn spawn<SC>(
        stream_consumer: SC,
        processor_thread: ProcessorThread<IS>,
    ) -> Result<ConsumerPool, ConsumerError>
    where
        SC: StreamConsumer<SM, SO, E, M, IS> + Send + Sync + 'static,
    {
        let consumer = Self::new(stream_consumer, processor_thread)?;
        let base_consumer = consumer.create_base_consumer()?;
        let offset_producer = consumer.create_offset_producer()?;

        let bc_offsets = consumer
            .get_latest_offsets(&base_consumer)?
            .into_iter()
            .map(
                |(tp, latest_offset)| -> Result<
                    (
                        Arc<BaseConsumer>,
                        StreamOffset,
                        Option<StreamOffset>,
                        Option<BaseProducer>,
                    ),
                    ConsumerError,
                > {
                    let bc = consumer.create_base_consumer()?;
                    consumer.assign_topic_partition(&bc, &tp)?;

                    let latest_consumed_offset =
                        ConsumerOffsets::load().lock().get(&tp).map(|o| o.clone());

                    let op = offset_producer.clone();

                    Ok((Arc::new(bc), latest_offset, latest_consumed_offset, op))
                },
            )
            .collect::<Result<
                Vec<(
                    Arc<BaseConsumer>,
                    StreamOffset,
                    Option<StreamOffset>,
                    Option<BaseProducer>,
                )>,
                ConsumerError,
            >>()?;

        let nb_consumers = bc_offsets.len();
        info!("{}: Starting {} consumer(s)", consumer.name(), nb_consumers);

        let active = Arc::new(AtomicUsize::new(0));
        let closed = Arc::new(AtomicUsize::new(0));
        let consumer = Arc::new(consumer);
        let name = consumer.name();
        let pool = ConsumerPool::new(nb_consumers)?;
        for (bc, latest_offset, latest_consumed_offset, op) in bc_offsets {
            let active = active.clone();
            let closed = closed.clone();
            let consumer = consumer.clone();
            pool.spawn(move |stop| {
                let ex = Executor::new();
                let local_ex = LocalExecutor::new();
                let nb_async_threads = 4;
                let (trigger, shutdown) = unbounded::<()>();
                Parallel::new()
                    .each(0..nb_async_threads, |_| ex.run(shutdown.recv()))
                    .finish(|| {
                        ex.enter(|| {
                            local_ex.run(async {
                                // drop at the end of this block
                                let _trigger = trigger;
                                active.fetch_add(1, Ordering::SeqCst);

                                if active.load(Ordering::Acquire) == 1 {
                                    debug!("{}: Starting", name);
                                    let consumer_event = ConsumerEvent::Starting(name.to_owned());
                                    let res = match consumer.processor_thread {
                                        ProcessorThread::Main(ref processor) => {
                                            processor.notify(consumer_event).map_err(|source| {
                                                ConsumerError::ConsumerProcessorNotify { source }
                                            })
                                        }
                                        ProcessorThread::Spawned(ref processor_sender) => {
                                            processor_sender
                                                .send(ProcessorMessage::Event(consumer_event))
                                                .await
                                                .map_err(|e| {
                                                    ConsumerError::ConsumerProcessorSender {
                                                        source: anyhow!(e),
                                                    }
                                                })
                                        }
                                    };

                                    if let Err(e) = res {
                                        error!(
                                            "{}: Unable to send close signal to processor: {:?}",
                                            name, e
                                        );
                                    }
                                }

                                loop {
                                    if *stop.read() {
                                        break;
                                    }
                                    if consumer.config.halt == ConsumerHalt::Auto
                                        && latest_offset.is_invalid()
                                    {
                                        break;
                                    }
                                    if let Some(msg) = bc.poll(Timeout::Never) {
                                        match ConsumerStream::try_from(msg) {
                                            Ok(stream) => {
                                                match consumer
                                                    .consume(
                                                        bc.clone(),
                                                        stream,
                                                        &latest_offset,
                                                        latest_consumed_offset.as_ref(),
                                                        op.as_ref(),
                                                    )
                                                    .await
                                                {
                                                    Ok(ConsumerLoop::Continue) => continue,
                                                    Ok(ConsumerLoop::Stop) => break,
                                                    Err(e) => error!("{}: {:?}", name, e),
                                                }
                                            }
                                            Err(e) => error!(
                                                "{}: Unable to decode consumer stream: {:?}",
                                                name, e
                                            ),
                                        }
                                    }
                                }

                                closed.fetch_add(1, Ordering::SeqCst);

                                if closed.load(Ordering::Acquire) == nb_consumers {
                                    debug!("{}: Terminating", name);
                                    let consumer_event =
                                        ConsumerEvent::Terminating(name.to_owned());
                                    let res = match consumer.processor_thread {
                                        ProcessorThread::Main(ref processor) => {
                                            processor.notify(consumer_event).map_err(|source| {
                                                ConsumerError::ConsumerProcessorNotify { source }
                                            })
                                        }
                                        ProcessorThread::Spawned(ref processor_sender) => {
                                            processor_sender
                                                .send(ProcessorMessage::Event(consumer_event))
                                                .await
                                                .map_err(|e| {
                                                    ConsumerError::ConsumerProcessorSender {
                                                        source: anyhow!(e),
                                                    }
                                                })
                                        }
                                    };

                                    if let Err(e) = res {
                                        error!(
                                            "{}: Unable to send close signal to processor: {:?}",
                                            name, e
                                        );
                                    }
                                }
                            })
                        })
                    });
            });
        }

        Ok(pool)
    }

    fn topic_partition_list(
        base_consumer: &BaseConsumer,
        topics: &[String],
    ) -> Result<Vec<TopicPartition>, ConsumerError> {
        let tps = topics
            .iter()
            .map(|topic| TopicPartition::topic_partition_list(base_consumer, topic))
            .collect::<Result<Vec<Vec<TopicPartition>>, KafkaError>>()
            .map_err(|source| ConsumerError::ConsumerTopicPartitionMetadata { source })?
            .into_iter()
            .flatten()
            .collect();

        Ok(tps)
    }
}
