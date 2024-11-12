use super::{ProducerConfig, ProducerConfigBuilder};
use super::{ProducerConfigError, ProducerError};
use super::{ProducerMessage, ProducerReceiver, ProducerSender};
use crate::common::TopicPartition;
use crate::stream::{OutputMessage, OutputStream};
use ahash::AHashMap;
use async_channel::{bounded, unbounded, Receiver, Sender};
use async_executor::{Executor, LocalExecutor};
use derivative::Derivative;
use easy_parallel::Parallel;
use log::{debug, error, info};
use rayon::ThreadPoolBuilder;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::base_consumer::BaseConsumer;
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::util::Timeout;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

pub trait StreamProducer: Send + Sync {
    fn config(
        &self,
        builder: &mut ProducerConfigBuilder,
    ) -> Result<ProducerConfig, ProducerConfigError>;
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Producer {
    config: ProducerConfig,
    #[derivative(Debug = "ignore")]
    stream_producer: Arc<Box<dyn StreamProducer>>,
}

impl Producer {
    fn create_consumer(&self) -> Result<BaseConsumer, ProducerError> {
        let config = &self.config;
        info!("{}: Create last index consumer", config.name);
        let mut kafka_config = ClientConfig::new();
        kafka_config
            .set("group.id", &config.group_id)
            .set("bootstrap.servers", &config.brokers.join(","))
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", &config.timeout.to_string())
            .set("enable.auto.commit", "false");

        if let Some(debug) = &config.debug {
            kafka_config.set("debug", &debug);
        }

        if let Some(level) = &config.log_level {
            kafka_config.set_log_level(*level);
        }

        let consumer: BaseConsumer = kafka_config
            .create()
            .map_err(|source| ProducerError::ProducerCreateClientConsumer { source })?;
        Ok(consumer)
    }

    fn create_producer(&self) -> Result<BaseProducer, ProducerError> {
        let config = &self.config;
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
            .map_err(|source| ProducerError::ProducerCreateClientProducer { source })?;

        Ok(producer)
    }

    fn get_topic_partition_list(&self) -> Result<Vec<TopicPartition>, ProducerError> {
        let consumer = self.create_consumer()?;

        info!("{}: Fetching topic partition metadata", self.name());
        let topic_partition_list =
            TopicPartition::topic_partition_list(&consumer, &self.config.topic)
                .map_err(|source| ProducerError::ProducerTopicPartitionMetadata { source })?;

        Ok(topic_partition_list)
    }

    pub fn name(&self) -> &'static str {
        self.config.name
    }

    fn new<SP>(stream_producer: SP) -> Result<Self, ProducerError>
    where
        SP: StreamProducer + 'static,
    {
        let mut builder = ProducerConfigBuilder::default();
        let config = stream_producer.config(&mut builder)?;

        Ok(Producer {
            config,
            stream_producer: Arc::new(Box::new(stream_producer)),
        })
    }

    pub fn spawn<SP, O, SM, SO, E, M>(
        stream_producer: SP,
        bound: Option<usize>,
    ) -> Result<ProducerSender<O, SM, SO, E, M>, ProducerError>
    where
        SP: StreamProducer + 'static,
        O: OutputStream<SM, SO, E, M> + Send + Sync + 'static,
        SM: Debug + Serialize + Send + Sync + 'static,
        SO: Debug + Serialize + Send + Sync + 'static,
        E: Debug + Serialize + Send + Sync + 'static,
        M: Debug + Serialize + Send + Sync + 'static,
    {
        let (sender, receiver): (
            ProducerSender<O, SM, SO, E, M>,
            ProducerReceiver<O, SM, SO, E, M>,
        ) = if let Some(max) = bound {
            bounded(max)
        } else {
            unbounded()
        };

        let producer = Self::new(stream_producer)?;
        let base_producer = producer.create_producer()?;

        let tp_list = producer.get_topic_partition_list()?;

        let mut sx_rx: AHashMap<
            TopicPartition,
            (
                Sender<OutputMessage<O, SM, SO, E, M>>,
                Receiver<OutputMessage<O, SM, SO, E, M>>,
            ),
        > = AHashMap::new();
        for tp in tp_list.iter() {
            sx_rx.insert(tp.clone(), bounded(1));
        }

        let nb_producers = tp_list.len();

        info!(
            "{}: Starting {} producer(s) thread",
            producer.name(),
            nb_producers
        );

        let producer = Arc::new(producer);
        let sx_rx = Arc::new(sx_rx);
        let name = producer.name();
        let pool = ThreadPoolBuilder::new()
            .num_threads(nb_producers + 1)
            .build()?;
        for tp in tp_list {
            let bp = base_producer.clone();
            let ps = producer.clone();
            let pr = producer.clone();
            let sx = sx_rx.clone();
            let rx = sx_rx.clone();
            let recv = receiver.clone();
            pool.spawn(move || {
                let ex = Executor::new();
                let local_ex = LocalExecutor::new();
                Parallel::new()
                    .add(|| {
                        let task = ex.spawn(async move {
                            if let Some((_, rx)) = rx.get(&tp) {
                                while let Ok(output_message) = rx.recv().await {
                                    if let Err(e) =
                                        ps.stream(bp.clone(), output_message, tp.clone())
                                    {
                                        error!("{}: {:?}", name, e);
                                    };
                                }
                            }
                        });
                        ex.run(task)
                    })
                    .finish(|| {
                        ex.enter(|| {
                            local_ex.run(async move {
                                while let Ok(msg) = recv.recv().await {
                                    match msg {
                                        ProducerMessage::Drop => break,
                                        ProducerMessage::Output(output_message) => {
                                            match output_message.stream().key() {
                                                Ok(key) => {
                                                    let tp = TopicPartition::from_key(
                                                        &key,
                                                        nb_producers,
                                                        &pr.config.topic,
                                                    );

                                                    if let Some((sx, _)) = sx.get(&tp) {
                                                        if let Err(e) =
                                                            sx.send(output_message).await
                                                        {
                                                            error!(
                                                            "{}: Unable to stream message: {:?}",
                                                            name, e
                                                        )
                                                        }
                                                    }
                                                }
                                                Err(e) => error!(
                                                    "{}: Unable to get stream key: {:?}",
                                                    name, e
                                                ),
                                            }
                                        }
                                    }
                                }
                                drop(recv);
                            })
                        })
                    });
            });
        }

        pool.spawn(move || loop {
            base_producer.poll(Timeout::After(Duration::from_secs(0)));
        });

        Ok(sender)
    }

    fn stream<O, SM, SO, E, M>(
        &self,
        base_producer: BaseProducer,
        output_message: OutputMessage<O, SM, SO, E, M>,
        tp: TopicPartition,
    ) -> Result<(), ProducerError>
    where
        O: OutputStream<SM, SO, E, M> + Send + 'static,
        SM: Debug + Serialize + Send + 'static,
        SO: Debug + Serialize + Send + 'static,
        E: Debug + Serialize + Send + 'static,
        M: Debug + Serialize + Send + 'static,
    {
        let name = self.name();
        let key = output_message.stream().key()?;
        let payload = output_message
            .to_json(&self.config.format)?
            .ok_or_else(|| ProducerError::ProducerJsonSerializationUnsupported)?;

        debug!(
            "{}: Produce\nTopic: {}\nPartition: {}\nKey: {}\n{}",
            name,
            tp.topic(),
            tp.partition(),
            key,
            payload
        );
        base_producer
            .send::<String, String>(
                BaseRecord::to(tp.topic())
                    .partition(tp.partition())
                    .key(&key)
                    .payload(&payload),
            )
            .map_err(|(source, msg)| ProducerError::ProducerSendRecord {
                source,
                msg: format!("{:?}", msg),
            })?;

        if let Some(next) = output_message.next() {
            next.next();
        }
        Ok(())
    }
}
