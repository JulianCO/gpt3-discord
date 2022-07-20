FROM rust:1.62-buster as builder
WORKDIR /usr/src/
# create blank project
RUN USER=root cargo new gpt3-discord
WORKDIR /usr/src/gpt3-discord

# Dummy build to cache dependencies
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

COPY ./src ./src
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/gpt3-discord /usr/local/bin/gpt3-discord
CMD ["gpt3-discord"]
