# TODO - Deconstruct the environment into seperate build-stages for external targetting
FROM jo3mccain/rusty as builder-base

ADD . /project
WORKDIR /project

COPY . .
RUN cargo test --workspace

FROM builder-base as builder

RUN cargo build --release -p acme-api && \
    cargo build --release -p acme-cli && \
    cargo package -p acme --features full && \
    cargo publish --

FROM debian:buster-slim as application

ENV DEV_MODE = false \
    MAINNET_PORT = 9999 \
    NODE_PORT = 9990 \
    SUBNET_PORT = 9900 \
    TCP_ACCESS_POINT = 9000 \
    SERVER_PORT = 8888 \
    REVERSE_PROXY_PORT = 8080

COPY --from=builder /bin/target/release/acme-api /bin/acme-api
COPY --from=builder /bin/target/release/acme-cli /bin/acme-cli

EXPOSE 8888:${SERVER_PORT}
ENTRYPOINT ["./bin/acme-api"]
ENTRYPOINT ["./bin/acme-cli"]