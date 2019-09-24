Rust CQRS+ES Example
=============================

    $ docker-compose up
    $ docker exec -it rust-cqrses-bankaccount_app_1 sh

    $ cargo run dynamodb
    $ aws dynamodb scan --table-name event_store --endpoint-url http://db:8000
