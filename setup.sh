#!/bin/bash 

docker compose up -d \
  postgres \
  zookeeper \
  kafka \
  fe-server-2 \
  be-server-2 \
  cn-server-2 \
  jobmanager \
  taskmanager
