# Starrocks-Cluster-on-Docker

## Prerequisite
1. Download a file starrocks in *tar.gz* format. Follow the command below
```bash
wget https://releases.starrocks.io/starrocks/StarRocks-2.5.16.tar.gz -P ./image/base/
```

2. Create docker network
```bash
docker network create <network_name>
# For example
docker network create starrocks-net
```

3. Find the subnet of docker network. Follow the command below
```bash
docker network inspect <network_name>
# Find the field of 'Subnet' and then copy the value
```

4. Fill *priority-networks* field in **be.conf** and **fe.conf** with value of subnet docker network. You can find the file in *image/base* directory

5. Create docker image base.
```bash
./build_base.sh
```

6. Create docker volume (Assumption the cluster using 1 FE and 1 BE)
```bash
docker volume create starrocks-fe
docker volume create starrocks-be
```

7. Start your Starrocks Cluster
```bash
docker compose up -d --build
```

## Additional
Instruction above if your cluster using 1 FE and 1 BE. So if you want your cluster using 1 FE and 3 BE or more of backend server. First of all you must create docker volume in accordance plan of your cluster (1 FE and 3 BE).
```bash
docker volume create sr-fe
docker volume create sr-be-1
docker volume create sr-be-2
docker volume create sr-be-3
```
If you done of create docker volume you must modification *docker-compose.yml* syntax.
```docker
version: '3.3'

networks:
  starrocks-net:
    driver: bridge
    external: true

services:
  fe-server:
    image: starrocks-fe 
    container_name: fe-server
    build:
      context: ./image/fe
      dockerfile: Dockerfile.fe
    volumes:
      - sr-fe:/opt/starrocks/fe
    ports:
      - 8030:8030
      - 9030:9030
    networks:
      - starrocks-net
  
  be-server-1:
    image: starrocks-be
    container_name: be-server-1
    build:
      context: ./image/be
      dockerfile: Dockerfile.be
    volumes:
      - sr-be-1:/opt/starrocks/be
    ports:
      - 8041:8040
      - 9061:9060
    networks:
      - starrocks-net
    depends_on:
      - fe-server

  be-server-2:
    image: starrocks-be
    container_name: be-server-2
    build:
      context: ./image/be
      dockerfile: Dockerfile.be
    volumes:
      - sr-be-2:/opt/starrocks/be
    ports:
      - 8042:8040
      - 9062:9060
    networks:
      - starrocks-net
    depends_on:
      - fe-server

  be-server-3:
    image: starrocks-be
    container_name: be-server-3
    build:
      context: ./image/be
      dockerfile: Dockerfile.be
    volumes:
      - sr-be-3:/opt/starrocks/be
    ports:
      - 8043:8040
      - 9063:9060
    networks:
      - starrocks-net
    depends_on:
      - fe-server

volumes:
  sr-fe:
    external: true
  sr-be-1:
    external: true
  sr-be-2:
    external: true
  sr-be-3:
    external: true
```
