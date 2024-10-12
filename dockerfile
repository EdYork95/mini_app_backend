# Start from an official Rust image
FROM rust:1.81.0

WORKDIR /mini_app_backend

COPY . .

RUN apt-get update

RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

RUN apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY /target/release/mini_app_backend /usr/local/bin/mini_app_backend

EXPOSE 8080

CMD ["mini_app_backend"]
