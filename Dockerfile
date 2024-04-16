FROM rust:latest as builder
WORKDIR /build
COPY . /build
RUN cargo build --bin cafard --release
RUN cargo build --bin cafard-server --release

COPY --from=builder /build/target/release/cafard .
COPY --from=builder /build/target/release/cafard-server .

CMD ["/cafard-server"]