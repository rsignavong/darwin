use super::{
    component::Component,
    error::{CliError, Result},
};
use derive_new::new;
use heck::ToSnakeCase;
use inflector::string::pluralize::to_plural;
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process::{Child, Command},
    thread::sleep,
    time::Duration,
};
use tracing::info;

#[derive(Deserialize)]
struct SetupYaml {
    kafka: SetupYamlKafka,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetupYamlKafka {
    topics_to_delete: Vec<String>,
    topics: Vec<String>,
}

#[derive(new)]
pub struct Kafka {
    project_path: PathBuf,
    kafka_broker: String,
}

impl Kafka {
    fn create_topic(
        &self,
        kafka_topics_cmd: &Path,
        replication_factor: &str,
        topic: &str,
    ) -> Result<Child> {
        let create = Command::new(kafka_topics_cmd)
            .args([
                "--bootstrap-server",
                &self.kafka_broker,
                "--create",
                "--replication-factor",
                replication_factor,
                "--topic",
                topic,
            ])
            .spawn()?;

        Ok(create)
    }

    fn check_topic_exists(
        &self,
        kafka_topics_cmd: &Path,
        kafka_topics_to_delete: &[String],
        topic: &str,
    ) -> Result<bool> {
        let topics = self.list_kafka_topics(kafka_topics_cmd, kafka_topics_to_delete)?;

        Ok(topics.contains(topic))
    }

    pub fn create_topic_if_needed(
        &self,
        kafka_topics_cmd: PathBuf,
        kafka_topic_replication_factor: u8,
        environment: String,
        component: &Component,
    ) -> Result<()> {
        info!("Create topic");

        let mut setup_yaml_path = self.project_path.clone();
        setup_yaml_path.push(format!("bootstrap/setup.{}.yaml", environment));

        let setup_yaml_file = setup_yaml_path.canonicalize()?;
        let mut file = File::open(setup_yaml_file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let setup_yaml: SetupYaml = serde_yaml::from_slice(&buffer)?;
        let topics_set: HashSet<String> = HashSet::from_iter(setup_yaml.kafka.topics);

        let type_ = to_plural(
            component
                .get_type()
                .ok_or_else(|| CliError::ComponentNotTyped(component.get_raw().to_owned()))?,
        );

        if component
            .get_type()
            .map(|s| s != "Query")
            .unwrap_or_else(|| false)
        {
            let found_topic = topics_set
                .iter()
                .find(|topic| {
                    topic.contains(&type_.to_snake_case())
                        && topic.contains(&to_plural(component.get_entity()).to_snake_case())
                })
                .ok_or_else(|| {
                    CliError::KafkaTopicNotFound(
                        type_.to_snake_case(),
                        to_plural(component.get_entity()).to_snake_case(),
                    )
                })?;

            if self.check_topic_exists(
                &kafka_topics_cmd,
                &setup_yaml.kafka.topics_to_delete,
                found_topic,
            )? {
                info!("Topic {} already exists", found_topic);
            } else {
                let mut kafka_topics = self.create_topic(
                    &kafka_topics_cmd,
                    &kafka_topic_replication_factor.to_string(),
                    found_topic,
                )?;
                kafka_topics.wait()?;

                info!("Topic {} created", found_topic);
            }
        } else {
            info!("Query processor. No topic to create");
        }

        Ok(())
    }

    pub fn create_topics(
        &self,
        kafka_topics_cmd: PathBuf,
        kafka_topic_replication_factor: u8,
        limit: u8,
        environment: String,
    ) -> Result<()> {
        info!("Create topics in kafka");

        let mut setup_yaml_path = self.project_path.clone();
        setup_yaml_path.push(format!("bootstrap/setup.{}.yaml", environment));

        let setup_yaml_file = setup_yaml_path.canonicalize()?;
        let mut file = File::open(setup_yaml_file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let setup_yaml: SetupYaml = serde_yaml::from_slice(&buffer)?;
        let topics_set: HashSet<String> = HashSet::from_iter(setup_yaml.kafka.topics.clone());

        info!("Removing topics");
        for (i, topic) in setup_yaml.kafka.topics_to_delete.iter().enumerate() {
            let mut kafka_topics = Command::new(&kafka_topics_cmd)
                .args([
                    "--bootstrap-server",
                    &self.kafka_broker,
                    "--delete",
                    "--topic",
                    topic,
                ])
                .spawn()?;
            if setup_yaml.kafka.topics_to_delete.len() - 1 == i {
                kafka_topics.wait()?;
            }
        }

        while !self
            .list_kafka_topics(&kafka_topics_cmd, &setup_yaml.kafka.topics_to_delete)?
            .is_disjoint(&topics_set)
        {
            info!("Waiting for kafka to delete topics...");
            sleep(Duration::from_millis(1_000));
        }

        info!("Creating topics");
        self.create_topics_by_chunks(
            &setup_yaml.kafka.topics,
            &kafka_topics_cmd,
            kafka_topic_replication_factor,
            limit,
        )?;

        let kafka_topics_list =
            self.list_kafka_topics(&kafka_topics_cmd, &setup_yaml.kafka.topics_to_delete)?;
        let diff_topics = |topics_set: &HashSet<String>,
                           kafka_topics_list: &HashSet<String>|
         -> HashSet<String> {
            topics_set
                .difference(kafka_topics_list)
                .collect::<HashSet<_>>()
                .iter()
                .map(|s| s.to_string())
                .collect()
        };
        let mut missing_topics = diff_topics(&topics_set, &kafka_topics_list);
        while !missing_topics.is_empty() {
            info!(
                "Some errors occured... Creating {} missing topics",
                missing_topics.len()
            );
            self.create_topics_by_chunks(
                &missing_topics.into_iter().collect::<Vec<String>>(),
                &kafka_topics_cmd,
                kafka_topic_replication_factor,
                limit,
            )?;
            let kafka_topics_list =
                self.list_kafka_topics(&kafka_topics_cmd, &setup_yaml.kafka.topics_to_delete)?;
            missing_topics = diff_topics(&topics_set, &kafka_topics_list);
        }

        info!(
            "{}/{} Topics created in kafka \u{1f43c}!",
            self.list_kafka_topics(&kafka_topics_cmd, &setup_yaml.kafka.topics_to_delete)?
                .len(),
            topics_set.len()
        );

        Ok(())
    }

    fn create_topics_by_chunks(
        &self,
        topics: &[String],
        kafka_topics_cmd: &Path,
        kafka_topic_replication_factor: u8,
        limit: u8,
    ) -> Result<()> {
        for topics_by_chunks in topics.chunks(limit as usize) {
            for (i, topic) in topics_by_chunks.iter().enumerate() {
                let mut kafka_topics = self.create_topic(
                    kafka_topics_cmd,
                    &kafka_topic_replication_factor.to_string(),
                    topic,
                )?;
                if topics_by_chunks.len() - 1 == i {
                    info!("Waiting for topics creation");
                    kafka_topics.wait()?;
                }
            }
        }

        Ok(())
    }

    fn list_kafka_topics(
        &self,
        kafka_topics_cmd: &Path,
        kafka_topics_to_delete: &[String],
    ) -> Result<HashSet<String>> {
        let topics_list = Command::new(kafka_topics_cmd)
            .args(["--bootstrap-server", &self.kafka_broker, "--list"])
            .output()?;
        let topics_stdout = String::from_utf8(topics_list.stdout)
            .map_err(|source| CliError::KafkaTopicsListOutput { source })?;
        let topics: Vec<String> = topics_stdout
            .split('\n')
            .filter_map(|topic| {
                if topic.contains("procon")
                    || kafka_topics_to_delete.iter().any(|topic_to_delete| {
                        Regex::new(topic_to_delete)
                            .map(|re| re.is_match(topic))
                            .unwrap_or_else(|_| false)
                    })
                {
                    Some(topic.to_owned())
                } else {
                    None
                }
            })
            .collect();

        Ok(HashSet::from_iter(topics))
    }

    pub fn list_kafka_consumer_groups(
        &self,
        kafka_consumer_groups_cmd: &Path,
    ) -> Result<HashSet<String>> {
        let consumer_groups_list = Command::new(kafka_consumer_groups_cmd)
            .args(["--bootstrap-server", &self.kafka_broker, "--list"])
            .output()?;
        let consumer_groups_stdout = String::from_utf8(consumer_groups_list.stdout)
            .map_err(|source| CliError::KafkaConsumerGroupsListOutput { source })?;
        let consumer_groups: Vec<String> = consumer_groups_stdout
            .split('\n')
            .filter_map(|consumer_group| {
                if consumer_group.contains("Allium") || consumer_group.contains("Procon") {
                    Some(consumer_group.to_owned())
                } else {
                    None
                }
            })
            .collect();

        Ok(HashSet::from_iter(consumer_groups))
    }

    fn reset_offset(
        &self,
        kafka_consumer_groups_cmd: &Path,
        consumer_group: &str,
    ) -> Result<Child> {
        info!("Reset offsets for consumer group {consumer_group}");

        let reset = Command::new(kafka_consumer_groups_cmd)
            .args([
                "--bootstrap-server",
                &self.kafka_broker,
                "--group",
                consumer_group,
                "--delete",
            ])
            .spawn()?;

        Ok(reset)
    }

    pub fn reset_offsets(
        &self,
        kafka_consumer_groups_cmd: PathBuf,
        component: &Component,
    ) -> Result<()> {
        info!("Reset offsets");

        let type_ = to_plural(
            component
                .get_type()
                .ok_or_else(|| CliError::ComponentNotTyped(component.get_raw().to_owned()))?,
        );
        let kafka_consumer_groups_list =
            self.list_kafka_consumer_groups(&kafka_consumer_groups_cmd)?;
        let consumer_group = kafka_consumer_groups_list.iter().find(|consumer_group| {
            consumer_group.contains(&type_)
                && consumer_group.contains(&to_plural(component.get_entity()))
        });

        match consumer_group {
            Some(cg) => {
                let mut reset_offset = self.reset_offset(&kafka_consumer_groups_cmd, cg)?;
                reset_offset.wait()?;

                info!("Reset offsets done \u{1f43c}!");
            }
            None => {
                info!(
                    "Kafka reset offset unknown consumer group: {}",
                    component.get_raw().to_owned()
                );
            }
        }

        Ok(())
    }
}
