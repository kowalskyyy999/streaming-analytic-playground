FROM ubuntu:22.04

ARG KAFKA_VERSION
ARG SCALA_VERSION

RUN apt-get clean

ENV TZ="Asia/jakarta"

RUN apt-get update && \
    apt-get upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y curl wget software-properties-common

RUN apt-get install -y openjdk-8-jdk-headless

ENV JAVA_HOME="/usr/lib/jvm/java-1.8.0-openjdk-amd64"

RUN wget https://archive.apache.org/dist/kafka/${KAFKA_VERSION}/kafka_${SCALA_VERSION}-${KAFKA_VERSION}.tgz

RUN tar -zxvf kafka_${SCALA_VERSION}-${KAFKA_VERSION}.tgz && \
    mv kafka_${SCALA_VERSION}-${KAFKA_VERSION} /usr/local/etc/kafka && \
    mkdir -p /usr/local/etc/kafka/logs && \
    mkdir -p /usr/local/etc/kafka/plugins && \
    rm kafka_${SCALA_VERSION}-${KAFKA_VERSION}.tgz

ENV KAFKA_HOME=/usr/local/etc/kafka
ENV PATH="$PATH:${KAFKA_HOME}/bin"

COPY config/server.properties ${KAFKA_HOME}/config/server.properties
COPY config/connect-distributed.properties ${KAFKA_HOME}/config/connect-distributed.properties

WORKDIR ${KAFKA_HOME}

CMD source ~/.bashrc
