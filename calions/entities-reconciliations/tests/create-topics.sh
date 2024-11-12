#!/bin/sh

FILE="./$1/topics.csv" 

KAFKA_BROKER=${2:-localhost:9092}
KAFKA_TOPICS_CMD=${3:-kafka-topics}
REPLICATION_FACTOR=${4:-1}

IFS=";"

[ ! -f $FILE ] && { echo "$FILE file not found"; exit 99; }

while read line
do
    TOPIC="calions-int-evt-$line"
    $KAFKA_TOPICS_CMD --bootstrap-server $KAFKA_BROKER --create -replication-factor $REPLICATION_FACTOR --partitions 3 --topic $TOPIC
done < $FILE

sleep 1


