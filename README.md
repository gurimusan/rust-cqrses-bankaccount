Rust CQRS+ES Example
=============================

    $ docker-compose up
    $ docker exec -it rust-cqrses-bankaccount_app_1 sh

Mysql
-----

    $ diesel setup

    $ kafka-topics.sh --zookeeper zoo:2181 --create --replication-factor 1 --partitions 1 --topic bank_account

    $ cargo run --bin snapshot_runner

    $ cargo run --bin grpc_server -- \
      --host 127.0.0.1 \
      --port 8000

    $ cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      open foo

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      close "055c48a7-2277-420e-8470-fcbd3d14b83f"

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      update "055c48a7-2277-420e-8470-fcbd3d14b83f" "foo updated"

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      deposit "055c48a7-2277-420e-8470-fcbd3d14b83f" 1000

    $ RUST_BACKTRACE=1 cargo run --bin grpc_client -- \
      --host 127.0.0.1 \
      --port 8000 \
      withdraw "055c48a7-2277-420e-8470-fcbd3d14b83f" 300
