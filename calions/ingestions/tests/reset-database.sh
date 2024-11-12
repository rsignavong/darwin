#!/bin/sh

DATABASE=$1
HOST=${3:-localhost}
PORT=${4:-5432}
USER=${2:-postgres}

dropdb -h $HOST -p $PORT -U $USER $DATABASE

