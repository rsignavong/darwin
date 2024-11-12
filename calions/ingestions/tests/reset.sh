#!/bin/sh

DATABASE=$1
PG_USER=$2

echo "Reset Database"
sh ./reset-database.sh $DATABASE $PG_USER
sh ./create-database.sh $DATABASE $PG_USER

echo "Reset Topics"
sh ./reset-topics.sh
sh ./create-topics.sh

echo "Done"
