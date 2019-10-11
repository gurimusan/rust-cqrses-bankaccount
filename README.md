Rust CQRS+ES Example
=============================

    $ docker-compose up
    $ docker exec -it rust-cqrses-bankaccount_app_1 sh

    $ aws dynamodb create-table --cli-input-json file://./examples/aws/dynamodb/event_store.json --endpoint-url http://db:8000
    $ aws dynamodb create-table --cli-input-json file://./examples/aws/dynamodb/snapshot.json --endpoint-url http://db:8000

    $ cargo run --bin snapshot_runner -- \
      --dynamodb-endpoint http://db:8000 \
      --dynamodb-region ap-northeast-1 \
      --kafka-brokers kafka:9092 \
      --kafka-consume-group bank_account_snapshotter

    $ aws dynamodb scan --table-name event_store --endpoint-url http://db:8000
    $ aws dynamodb scan --table-name snapshot --endpoint-url http://db:8000

    $ aws dynamodb delete-table --table-name event_store --endpoint-url http://db:8000
    $ aws dynamodb delete-table --table-name snapshot --endpoint-url http://db:8000

    $ kafka-topics.sh --zookeeper zoo:2181 --create --replication-factor 1 --partitions 1 --topic bank_account
    $ kafka-topics.sh --list --zookeeper zoo:2181
    $ kafka-console-producer.sh --broker-list :9092 --topic bank_account

    kafka-console-producer.sh --broker-list localhost:9092 --topic

    kafka-console-producer.sh --broker-list 192.168.99.100:9092 --topic bank_account

    $ RUST_BACKTRACE=1 cargo run --bin grpc_server -- \
      --host 127.0.0.1 \
      --port 8000 \
      --dynamodb-endpoint http://db:8000 \
      --dynamodb-region ap-northeast-1 \
      --kafka-brokers kafka:9092


    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      open foo

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      close "3486e2d6-c93f-4ce6-8eec-a3a00c7126ae"

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      update "341d6356-6dd6-4055-99eb-7ca046ac2dc4" "foo updated"

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      deposit "341d6356-6dd6-4055-99eb-7ca046ac2dc4" 1000

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      withdraw "341d6356-6dd6-4055-99eb-7ca046ac2dc4" 300
