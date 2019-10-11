FROM rustlang/rust:nightly-stretch-slim

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        groff-base \
        librdkafka-dev \
        awscli \
        gcc \
        libc6-dev \
        make \
        cmake \
        g++ \
        golang-go \
        libprotobuf-dev \
        libprotoc-dev \
        protobuf-compiler \
        ; \
    rm -rf /var/lib/apt/lists/*;
