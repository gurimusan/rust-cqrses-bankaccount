FROM rustlang/rust:nightly-stretch-slim

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        groff-base \
        awscli \
        ; \
    rm -rf /var/lib/apt/lists/*;
