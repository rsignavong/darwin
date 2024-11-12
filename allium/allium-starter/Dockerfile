# syntax=docker/dockerfile:experimental
FROM rust:1.66-alpine3.17 AS starter
LABEL maintainer=allium

ARG GITLAB_TOKEN

RUN apk add git alpine-sdk g++ make bash cmake openssh openssl-dev 
RUN mkdir -m 700 /root/.ssh; \
  touch -m 600 /root/.ssh/known_hosts; \
  ssh-keyscan github.com > /root/.ssh/known_hosts \
  ssh-keyscan gitlab.com >> /root/.ssh/known_hosts
RUN git config --global credential.helper store
RUN echo "https://rsignavong:${GITLAB_TOKEN}@gitlab.com" > ~/.git-credentials
RUN mkdir -m 700 /root/.cargo; \
  touch -m 600 /root/.cargo/config; \
  echo '[net]' > /root/.cargo/config; \
  echo 'git-fetch-with-cli = true' >> /root/.cargo/config

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo install --git https://gitlab.com/rsignavong/allium_starter.git
