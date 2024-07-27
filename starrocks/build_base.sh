#!/bin/bash

set -e

docker build \
    -t starrocks-base:3.2.8 \
    -f ./image/base/Dockerfile .
