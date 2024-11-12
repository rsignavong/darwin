#!/bin/sh

CONTEXT=$1
TEST=$2

echo "Running Tests #$TEST"
sh ./run-test.sh $CONTEXT $TEST

echo "Done"
