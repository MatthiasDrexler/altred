FROM ekidd/rust-musl-builder:stable as builder
WORKDIR /rust/src/altred/microservices/common/backend/
COPY common/backend/ ./
RUN sudo chown -R rust:rust .
WORKDIR /rust/src/altred/microservices/user-management/backend
COPY user-management/backend/ ./
RUN sudo chown -R rust:rust .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3 as runner
WORKDIR /usr/bin/
COPY --from=builder /rust/src/altred/microservices/user-management/backend/target/x86_64-unknown-linux-musl/release/user-management /usr/bin/user-management
RUN addgroup --system nonrootgroup && adduser --system nonrootuser --ingroup nonrootgroup
USER nonrootuser
CMD ["user-management"]
