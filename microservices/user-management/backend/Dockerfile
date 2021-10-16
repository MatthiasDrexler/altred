FROM rust:slim-bullseye as builder
WORKDIR /rust/src/altred/microservices/common/backend/autowire_derive
COPY common/backend/autowire_derive/src/ ./src/
COPY common/backend/autowire_derive/Cargo.* ./
WORKDIR /rust/src/altred/microservices/user-management/backend
COPY user-management/backend/src/ ./src/
COPY user-management/backend/Cargo.* ./
RUN apt-get update && apt-get install --yes libpq5 libpq-dev
RUN cargo install --path .

FROM rust:slim-bullseye
COPY --from=builder /usr/local/cargo/bin/user-management /usr/local/cargo/bin/user-management
RUN apt-get update && apt-get install --yes libpq5 libpq-dev
RUN useradd -m nonrootuser
USER nonrootuser
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
CMD ["user-management"]
