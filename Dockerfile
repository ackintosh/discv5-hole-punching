# syntax=docker/dockerfile:1
FROM rust:1.75.0-bookworm

ARG DEBIAN_FRONTEND=noninteractive
RUN --mount=type=cache,target=/var/cache/apt apt-get update && apt-get -y install iproute2 iputils-ping dnsutils netcat-traditional redis-tools

WORKDIR /workspace
ADD . .

RUN --mount=type=cache,target=./target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

RUN --mount=type=cache,target=./target \
    mv ./target/release/discv5-hole-punching /usr/local/bin/discv5-hole-punching

#ENTRYPOINT ["/usr/local/bin/discv5-hole-punching"]

#FROM python:3

#ARG DEBIAN_FRONTEND=noninteractive
#RUN --mount=type=cache,target=/var/cache/apt apt-get update && apt-get -y install iproute2 iputils-ping dnsutils netcat-traditional

COPY run.sh /scripts/
RUN chmod +x /scripts/*.sh

## debug
#COPY *.py /scripts/

ENTRYPOINT ["/scripts/run.sh"]
