FROM rust:1.42

WORKDIR /app

COPY . .

RUN cargo build --release

COPY target/release/storage .

CMD ["storage"]
