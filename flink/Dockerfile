FROM flink:1.17.0-scala_2.12-java11
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/com/ververica/flink-sql-connector-postgres-cdc/2.3.0/flink-sql-connector-postgres-cdc-2.3.0.jar
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-sql-connector-kafka/1.17.0/flink-sql-connector-kafka-1.17.0.jar
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-table-api-java/1.17.1/flink-table-api-java-1.17.1.jar 
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-table-api-java-bridge/1.17.1/flink-table-api-java-bridge-1.17.1.jar
RUN wget -P /opt/flink/lib https://repo1.maven.org/maven2/org/apache/flink/flink-table-common/1.17.1/flink-table-common-1.17.1.jar

RUN mkdir /opt/jobs
COPY /flink/job/job.sql /opt/jobs/job.sql
