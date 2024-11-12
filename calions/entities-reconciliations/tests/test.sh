#!/bin/sh

echo "Create Topics"
sh ./create-topics.sh $1

echo "Running Tests #$2"
sh ./run-test.sh $1 $2

echo "Done"
