FROM rust:slim-bullseye as builder
WORKDIR /rust/src/user-management
RUN apt-get update && apt-get install --yes libpq5 libpq-dev
COPY src/ ./src/
COPY Cargo.* ./
RUN cargo install --path .

FROM rust:slim-bullseye
COPY --from=builder /usr/local/cargo/bin/user-management /usr/local/cargo/bin/user-management
RUN apt-get update && apt-get install --yes libpq5 libpq-dev
RUN useradd -m nonrootuser
USER nonrootuser
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
CMD ["user-management"]
