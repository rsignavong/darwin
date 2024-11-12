use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Invalid bootstrap.config.json")]
    BootstrapConfigJson,
    #[error("Invalid bootstrap generator path {path}, provide one with --bootstrap-path or environment variable BOOTSTRAP_PATH")]
    BootstrapPath {
        path: String,
        source: std::io::Error,
    },
    #[error("IO Error")]
    IO {
        #[from]
        source: std::io::Error,
    },
    #[error("Json Error")]
    Json {
        #[from]
        source: serde_json::Error,
    },
    #[error("Invalid component: {0}")]
    ComponentInvalid(String),
    #[error("Missing component, provide one with --component option")]
    ComponentMissing,
    #[error("Component does not contains Command, Query or Operator: {0}")]
    ComponentNotTyped(String),
    #[error("Invalid component pattern")]
    ComponentPattern(#[from] regex::Error),
    #[error("Invalid environment")]
    Environment,
    #[error("Invalid kafka-consumer-groups command path {path}, provide one with --kafka-consumer-groups-cmd or environment variable KAFKA_CONSUMER_GROUPS_CMD")]
    KafkaConsumerGroupsCmdInvalid {
        path: String,
        source: std::io::Error,
    },
    #[error("Missing kafka-consumer-groups command path, provide one with --kafka-consumer-groups-cmd or environment variable KAFKA_CONSUMER_GROUPS_CMD")]
    KafkaConsumerGroupsCmdMissing,
    #[error("Kafka consumer groups list parsing error")]
    KafkaConsumerGroupsListOutput { source: std::string::FromUtf8Error },
    #[error("Kafka topic of type {0} and for entity {1} not found in topics list")]
    KafkaTopicNotFound(String, String),
    #[error("Invalid kafka-topics command path {path}, provide one with --kafka-topics-cmd or environment variable KAFKA_TOPICS_CMD")]
    KafkaTopicsCmdInvalid {
        path: String,
        source: std::io::Error,
    },
    #[error("Missing kafka-topics command path, provide one with --kafka-topics-cmd or environment variable KAFKA_TOPICS_CMD")]
    KafkaTopicsCmdMissing,
    #[error("Kafka topics list parsing error")]
    KafkaTopicsListOutput { source: std::string::FromUtf8Error },
    #[error("Invalid local config path {path}, provide one with --local-config-path or environment variable LOCAL_CONFIG_PATH")]
    LocalConfigPath {
        path: String,
        source: std::io::Error,
    },
    #[error("Invalid Postgres dump command path {path}, provide one with --pg-dump-cmd or environment variable PG_DUMP_CMD")]
    PgDumpCmdInvalid {
        path: String,
        source: std::io::Error,
    },
    #[error("Invalid Postgres dump file path {path}, provide one with --dump-file")]
    PgDumpFileInvalid {
        path: String,
        source: std::io::Error,
    },
    #[error("Invalid Postgres sql command path {path}, provide one with --psql-cmd or environment variable PSQL_CMD")]
    PsqlCmdInvalid {
        path: String,
        source: std::io::Error,
    },
    #[error("Postgres dump parsing error")]
    PgDumpOutput { source: std::string::FromUtf8Error },
    #[error("Invalid project path {path}, provide one with --project-path or environment variable PROJECT_PATH")]
    ProjectPath {
        path: String,
        source: std::io::Error,
    },
    #[error("Yaml Error")]
    Yaml {
        #[from]
        source: serde_yaml::Error,
    },
}

pub type Result<T> = std::result::Result<T, CliError>;
