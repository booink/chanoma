FROM rust:1.59.0-slim-bullseye

USER root

RUN apt update && apt install -y sudo

RUN apt-get update -y \
    && apt-get install -y libssl-dev pkg-config locales gnuplot \
    && locale-gen ja_JP.UTF-8

RUN localedef -f UTF-8 -i ja_JP ja_JP.UTF-8
ENV LANG="ja_JP.UTF-8" \
    LANGUAGE="ja_JP:ja" \
    LC_ALL="ja_JP.UTF-8"

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     python3 \
     python3-dev \
     python3-pip \
     ruby \
     ruby-dev \
     git \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

RUN gem install bundler

RUN rustup component add clippy
RUN cargo install cargo-watch cargo-make cargo-expand cargo-tarpaulin sccache

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
WORKDIR /app
COPY Cargo.toml Cargo.toml
ADD src src

ENV PATH=$PATH:/home/$USER/.local/bin \
    PYTHONPATH=/home/$USER/.local/lib/python3.7/site-packages/

COPY bindings/python3/requirements.txt bindings/python3/requirements.txt
RUN cd bindings/python3 && \
    python3 -m pip install --upgrade pip && \
    python3 -m pip install -r requirements.txt

ADD . .
