FROM jo3mccain/rusty:latest as builder

ADD . /bin
WORKDIR /bin

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /bin/target/release/acme-cli /bin/acme-cli

ENTRYPOINT ["./project/acme-cli"]