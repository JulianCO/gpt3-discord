FROM rust:1.62-buster as builder
WORKDIR /usr/src/gpt3-discord
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/gpt3-discord /usr/local/bin/gpt3-discord
CMD ["gpt3-discord"]
