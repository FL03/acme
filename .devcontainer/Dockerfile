# description: A Dockerfile defining the development environment for the project.
# file: .devcontainer/Dockerfile 

# STAGE 1: setup the base image
FROM debian:bookworm-slim AS devbase
# update & upgrade the system packages
RUN apt-get update -y && \
    apt-get upgrade -y
# install the necessary packages
RUN apt-get install -y \
    build-essential \
    clang \
    cmake \
    git \
    libopenblas-dev \
    libssl-dev \
    pkg-config
# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

ENV PATH="/root/.cargo/bin:${PATH}" \
    RUSTUP_HOME="/root/.rustup" \
    CARGO_HOME="/root/.cargo"

FROM devbase AS development
# set the environment variables for rust
ENV RUST_BACKTRACE=1 \
    CARGO_INCREMENTAL=0 \
    CARGO_BUILD_TARGET="x86_64-unknown-linux-musl" \
    CARGO_TARGET_DIR="/app/target" \
    RUSTFLAGS=""
# set the workdir
WORKDIR /app
# add the source code to the container
ADD . .
# configure the container entrypoint
ENTRYPOINT ["cargo"]
