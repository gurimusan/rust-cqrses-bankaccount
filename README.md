Rust CQRS+ES Example
=============================

Futures
-------

- Implement Rust
- Eventsourcing
- Command Query Responsibility Segregation
- EventStore using Mysql
- Event broker useing Kafka
- Query side data store using Elasticsearch

Run
---

### Run docker container:

    $ docker-compose up

### Create Kafka topic

    $ docker exec -it rust-cqrses-bankaccount_kafka_1 sh -c "kafka-topics.sh --zookeeper zoo:2181 --create --replication-factor 1 --partitions 1 --topic bank_account"

### Create Mysql table

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cd examples/mysql && diesel migration run"

### Cargo init

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo build"

### Run snapshotter

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin snapshot_runner"

### Run projector

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin projector_runner"

### Run gRPC Server

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin grpc_server -- --host 127.0.0.1 --port 8000"

Command example
---------------

Open:

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin grpc_client -- --host 127.0.0.1 --port 8000 open foo"

Update:

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin grpc_client -- --host 127.0.0.1 --port 8000 update <bank-account-id> 'foo updated'"

Deposit:

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin grpc_client -- --host 127.0.0.1 --port 8000 deposit <bank-account-id> 1000"

Withdraw:

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin grpc_client -- --host 127.0.0.1 --port 8000 withdraw <bank-account-id> 300"

Close:

    $ docker exec -it rust-cqrses-bankaccount_app_1 sh -c "cargo run --bin grpc_client -- --host 127.0.0.1 --port 8000 close <bank-account-id>"

TIPS
----

An error like the following occurred:

    elasticsearch_1  | ERROR: [2] bootstrap checks failed
    elasticsearch_1  | [1]: max virtual memory areas vm.max_map_count [65530] is too low, increase to at least [262144]
    elasticsearch_1  | [2]: the default discovery settings are unsuitable for production use; at least one of [discovery.seed_hosts, discovery.seed_providers, cluster.initial_master_nodes] must be configured

Change system parameter:

    $ echo "vm.max_map_count = 262144" >> /etc/sysctl.d/99-sysctl.conf
    $ sysctl --system
