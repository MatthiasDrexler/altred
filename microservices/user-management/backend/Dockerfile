FROM rust:slim-bullseye as builder
WORKDIR /rust/src/altred/microservices/common/backend/
COPY common/backend/ ./
WORKDIR /rust/src/altred/microservices/user-management/backend
COPY user-management/backend/ ./
CMD ["bash"]
# RUN apt-get update && apt-get install --yes libpq5 libpq-dev
# RUN cargo install --path .

# FROM rust:slim-bullseye
# COPY --from=builder /usr/local/cargo/bin/user-management /usr/local/cargo/bin/user-management
# RUN apt-get update && apt-get install --yes libpq5 libpq-dev
# RUN useradd -m nonrootuser
# USER nonrootuser
# ARG DATABASE_URL
# ENV DATABASE_URL=$DATABASE_URL
# CMD ["user-management"]
