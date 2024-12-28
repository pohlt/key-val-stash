FROM rust:1 as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

WORKDIR /app
COPY . /app
#RUN cargo build --release
RUN cargo build --target x86_64-unknown-linux-musl --release

#FROM gcr.io/distroless/cc-debian12
#COPY --from=build /app/target/release/key-val-stash /

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/key-val-stash /

CMD ["./key-val-stash"]
