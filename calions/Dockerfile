# syntax=docker/dockerfile:experimental

FROM rust:1-alpine3.11 as builder
MAINTAINER rsignavong@gmail.com

ARG PROCESSOR_NAME
ENV PROCESSOR_NAME=$PROCESSOR_NAME
RUN echo $PROCESSOR_NAME

RUN apk update && apk add git alpine-sdk g++ make bash cmake openssl-dev openssh
RUN mkdir -p /app/processors
RUN mkdir -p /app/dist
RUN mkdir -m 700 /root/.ssh; \
  touch -m 600 /root/.ssh/known_hosts; \
  ssh-keyscan github.com > /root/.ssh/known_hosts \
  ssh-keyscan gitlab.com >> /root/.ssh/known_hosts

COPY processors/$PROCESSOR_NAME /app/processors/$PROCESSOR_NAME

WORKDIR /app/processors/$PROCESSOR_NAME
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN --mount=type=ssh cargo build --release --target-dir /app/dist

FROM rust:1-alpine3.11
RUN apk update && apk add libgcc openssl

RUN mkdir -p /app
ARG PROCESSOR_NAME
ENV PROCESSOR_NAME=$PROCESSOR_NAME
COPY --from=builder --chown=nobody:nobody /app/dist/release/$PROCESSOR_NAME /app/server
WORKDIR /app
RUN mkdir data && chown nobody:nobody data
#USER nobody
CMD ["./server"]


