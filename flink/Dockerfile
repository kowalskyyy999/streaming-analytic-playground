FROM flink:1.17.0-scala_2.12-java11
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/com/ververica/flink-sql-connector-postgres-cdc/2.3.0/flink-sql-connector-postgres-cdc-2.3.0.jar
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-sql-connector-kafka/1.17.0/flink-sql-connector-kafka-1.17.0.jar
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-table-api-java/1.17.1/flink-table-api-java-1.17.1.jar 
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-table-api-java-bridge/1.17.1/flink-table-api-java-bridge-1.17.1.jar
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-table-common/1.17.1/flink-table-common-1.17.1.jar

RUN echo bash /opt/flink/bin/jobmanager.sh start > "/opt/flink/bin/start-jobmanager.sh" && \
    echo sleep 10 >> "/opt/flink/bin/start-jobmanager.sh" && \
    echo bash /opt/flink/bin/sql-gateway.sh start -Dsql-gateway.endpoint.rest.address=localhost >> "/opt/flink/bin/start-jobmanager.sh" && \
    echo latest_log = $(ls -t log/flink--standalonesession-*.log | head -1) >> "/opt/flink/bin/start-jobmanager.sh" && \
    echo tail -f "$latest_log" >> "/opt/flink/bin/start-jobmanager.sh" && \
    chmod a+x /opt/flink/bin/start-jobmanager.sh

RUN mkdir /app

COPY jobs/create-job.sh /app/create-job.sh

RUN chmod +x /app/create-job.sh && \
  echo bash /app/create-job.sh > "/app/submit-job.sh" && \
  echo bash /opt/flink/bin/sql-client.sh --file job.sql >> "/app/submit-job.sh" && \
  chmod a+x /app/submit-job.sh

RUN mkdir /opt/jobs
COPY ./job/job.sql /opt/jobs/job.sql
