#!/bin/sh

KAFKA_BROKER=${1:-localhost:9092}
KAFKA_TOPICS_CMD=${2:-kafka-topics}
REPLICATION_FACTOR=${3:-1}

$KAFKA_TOPICS_CMD --bootstrap-server $KAFKA_BROKER --delete --topic 'calions-int-evt-.*'

$KAFKA_TOPICS_CMD --bootstrap-server $KAFKA_BROKER --create -replication-factor $REPLICATION_FACTOR --partitions 3 --topic calions-int-evt-entities_reconciliations
$KAFKA_TOPICS_CMD --bootstrap-server $KAFKA_BROKER --create -replication-factor $REPLICATION_FACTOR --partitions 3 --topic calions-int-evt-entities_reconciliations_data
$KAFKA_TOPICS_CMD --bootstrap-server $KAFKA_BROKER --create -replication-factor $REPLICATION_FACTOR --partitions 3 --topic calions-int-evt-entities_reconciliations_command
$KAFKA_TOPICS_CMD --bootstrap-server $KAFKA_BROKER --create -replication-factor $REPLICATION_FACTOR --partitions 3 --topic calions-int-evt-entities_reconciliations_statuses

