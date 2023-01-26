FROM rust as builder
COPY . .
RUN apt-get update \
    && apt-get -y install musl-tools \
    && mkdir -p /tmp/ \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo install --root /tmp/ --path . --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /tmp/bin/rusthook /rusthook
ENTRYPOINT [ "/rusthook" ]