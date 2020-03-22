FROM rust:latest as builder

COPY . .

RUN cargo build --release

FROM rust:latest

COPY --from=builder /target/release/storage .

RUN ls -la /storage

EXPOSE 8080

ENTRYPOINT ["/storage"]
