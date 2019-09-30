Rust CQRS+ES Example
=============================

    $ docker-compose up
    $ docker exec -it rust-cqrses-bankaccount_app_1 sh

    $ aws dynamodb create-table --cli-input-json file://./rust_cqrses_bankaccount_aws_example/migration/20190922000000_create_event_store.json --endpoint-url http://db:8000
    $ cargo run rust_cqrses_bankaccount_example
    $ aws dynamodb scan --table-name event_store --endpoint-url http://db:8000
    $ aws dynamodb delete-table --table-name event_store --endpoint-url http://db:8000
