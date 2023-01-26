FROM rust as builder
COPY . .
RUN mkdir -p /tmp/ \
    && cargo install --root /tmp/ --path .

FROM scratch
COPY --from=builder /tmp/bin/rusthook /rusthook
ENTRYPOINT [ "/rusthook" ]