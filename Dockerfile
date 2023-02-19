#planner
FROM lukemathwalker/cargo-chef as planner
WORKDIR /app
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

#cacher
FROM lukemathwalker/cargo-chef as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

#builder
FROM rust:latest as builder
WORKDIR /app

COPY migrations.sql /app/migrations.sql
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .

RUN cargo build --release

#runtime
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/spectre spectre
COPY postgres.conf /etc/postgresql/postgresql.conf
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*

ENV DISCORD_TOKEN=MTA2NjM3NDgxNDU4OTI2ODAwOQ.G8j8Sf.xeajIMKBhebedN7HoG6QGKk30dCmj3ZLN6QCzU
ENTRYPOINT [ "./spectre" ]