# Start from an official Rust image
FROM rust:1.81.0 as builder

WORKDIR /mini_app_backend


COPY . .

RUN cargo build --release

FROM debian:bookworm

RUN apt-get update

RUN apt-get install postgresql -y

RUN apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /mini_app_backend/target/release/mini_app_backend /usr/local/bin/mini_app_backend

EXPOSE 8080

CMD ["mini_app_backend"]
