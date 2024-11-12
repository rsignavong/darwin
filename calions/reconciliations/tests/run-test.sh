#!/bin/sh

KAFKA_BROKER=${3:-localhost:9092}
TEST="./$1/test_$2.csv"
OLDIFS=$IFS
IFS=';'
[ ! -f $TEST ] && { echo "$TEST file not found"; exit 99; }

while read topic file event index
do
    JSON="./$1/data/$file.json"
    [ ! -f $JSON ] && { echo "$JSON file not found"; exit 99; }
	  echo "Topic : $topic"
	  echo "File : $JSON"
    echo $(json-minify $JSON) | sed "s/\"event\":\"event\"/\"event\":\"${event}\"/g" | sed "s/\"index\":0/\"index\":${index}/g" | kafkacat -P -b $KAFKA_BROKER -t calions-int-evt-$topic
    sleep 1
done < $TEST
IFS=$OLDIFS
