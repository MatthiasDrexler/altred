FROM rust:slim-bullseye as builder
WORKDIR /rust/src/user-management
COPY src/ ./src/
COPY Cargo.* ./
RUN cargo install --path .

FROM rust:slim-bullseye
COPY --from=builder /usr/local/cargo/bin/user-management /usr/local/cargo/bin/user-management
RUN useradd -m nonrootuser
USER nonrootuser
CMD ["user-management"]
