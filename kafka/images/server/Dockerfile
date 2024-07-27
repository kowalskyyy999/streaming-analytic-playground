ARG KAFKA_VERSION

FROM kafka:${KAFKA_VERSION}

COPY config/server.properties $KAFKA_HOME/config/server.properties

CMD $KAFKA_HOME/bin/kafka-server-start.sh $KAFKA_HOME/config/server.properties
