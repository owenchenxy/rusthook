FROM rust as builder
COPY . .
RUN mkdir -p /tmp/ \
    && cargo install --root /tmp/ --path .

FROM scratch
COPY --from=builder /tmp/rusthook /rusthook
ENTRYPOINT [ "/rusthook" ]