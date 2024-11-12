#!/bin/sh

KAFKA_BROKER=${1:-localhost:9092}
KAFKA_TOPICS_CMD=${2:-kafka-topics}
REPLICATION_FACTOR=${3:-1}

$KAFKA_TOPICS_CMD --bootstrap-server $KAFKA_BROKER --delete --topic 'calions-int-evt-.*'

