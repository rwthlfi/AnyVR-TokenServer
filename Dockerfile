# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /usr/src/tokenserver

COPY Cargo.toml ./
COPY src ./src
COPY .env ./

RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/tokenserver/target/release/tokenserver .
COPY --from=builder /usr/src/tokenserver/.env .

EXPOSE 3030

ENTRYPOINT ["tokenserver"]

