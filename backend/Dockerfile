FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN apt-get update && \
  apt-get install -y protobuf-compiler && \
  cargo clean && cargo build --release

FROM ubuntu:latest

# Fix the missing package installs here
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl unzip

WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 50051
ENTRYPOINT export DB_URL=surrealdb:8000 && export REDIS_URL=redis://redis_for_bacon && /app/backend

CMD tail -f /dev/null
