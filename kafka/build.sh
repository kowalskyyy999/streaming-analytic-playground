#!/bin/bash

. ./.env

docker build \
    --build-arg KAFKA_VERSION=$KAFKA_VERSION \
    --build-arg SCALA_VERSION=$SCALA_VERSION \
    -t kafka:$KAFKA_VERSION \
    ./images/base
