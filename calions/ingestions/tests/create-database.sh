#!/bin/sh

FILE="./init.sql" 
DATABASE=$1
HOST=${3:-localhost}
PORT=${4:-5432}
USER=${2:-postgres}

createdb -h $HOST -p $PORT -U $USER $DATABASE

psql -h $HOST -p $PORT -U $USER -d $DATABASE -f $FILE
