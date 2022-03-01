FROM rust:1.59.0-slim-bullseye

RUN apt update && apt install -y sudo

ENV USER devuser
ENV GROUP devuser

RUN useradd -m -u 1000 --shell /bin/sh -G sudo,root ${USER}
RUN echo '%sudo ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers

RUN apt-get update -y \
    && apt-get install -y libssl-dev pkg-config locales \
    && locale-gen ja_JP.UTF-8

RUN localedef -f UTF-8 -i ja_JP ja_JP.UTF-8
ENV LANG="ja_JP.UTF-8" \
    LANGUAGE="ja_JP:ja" \
    LC_ALL="ja_JP.UTF-8"

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     python3 \
     python3-pip \
     ruby \
     ruby-dev \
     git \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

RUN gem install bundler

USER $USER

RUN rustup component add clippy
RUN cargo install cargo-watch cargo-make cargo-expand cargo-tarpaulin sccache

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
WORKDIR /app
COPY --chown=${USER}:${GROUP} Cargo.toml Cargo.toml
ADD --chown=${USER}:${GROUP} src src
#RUN cargo build --bin chanoma

ENV PATH=$PATH:/home/$USER/.local/bin \
    PYTHONPATH=/home/$USER/.local/lib/python3.7/site-packages/

COPY bindings/python3/requirements.txt bindings/python3/requirements.txt
RUN cd bindings/python3 && \
    python3 -m pip install --upgrade pip && \
    python3 -m pip install -r requirements.txt

#RUN export
#RUN python3 -m site --user-base
#RUN mkdir /tmp/abc && touch /tmp/abc/input.json && /usr/bin/python3 /home/devuser/.local/lib/python3.7/site-packages/pip/_vendor/pep517/in_process/_in_process.py get_requires_for_build_wheel /tmp/abc
ADD --chown=${USER}:${GROUP} . .
#RUN cd bindings/python3 && \
#    python3 -m pip --verbose install --user -e .

USER root
CMD ["cargo", "watch", "-x", "test", "-x", "clippy"]
