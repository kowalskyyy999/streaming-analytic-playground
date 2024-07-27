FROM ubuntu:22.04

ENV TZ="Asia/Jakarta"

ARG USER=starrocks
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

RUN apt-get clean

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y curl wget software-properties-common openjdk-11-jdk-headless ca-certificates

ENV JAVA_HOME="/usr/lib/jvm/java-1.11.0-openjdk-amd64"
ENV STARROCKS_HOME="/opt/starrocks"

WORKDIR /opt

RUN groupadd --gid ${USER_UID} ${USER} && \
    useradd --uid ${USER_UID} --gid $USER_GID -M --shell /usr/sbin/nologin ${USER} && \
    usermod -aG root ${USER}

COPY ./image/base/StarRocks-3.2.8.tar.gz .

RUN tar -zxvf StarRocks-3.2.8.tar.gz && \
    mv StarRocks-3.2.8 starrocks && \
    rm StarRocks-3.2.8.tar.gz 

COPY ./image/base/conf/fe.conf /opt/starrocks/fe/conf
COPY ./image/base/conf/be.conf /opt/starrocks/be/conf

RUN chown -R ${USER_UID}:${USER_GID} ${STARROCKS_HOME}

USER root

RUN echo source ~/.bashrc >> "entrypoint.sh" && \
    chmod a+x entrypoint.sh

CMD ./entrypoint.sh
