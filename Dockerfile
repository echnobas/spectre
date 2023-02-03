FROM rust:latest as rust
RUN cargo install cargo-chef 
WORKDIR /app

FROM rust as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
RUN cargo build

FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/debug/spectre spectre
COPY postgres.conf /etc/postgresql/postgresql.conf
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*

ENV DISCORD_TOKEN=MTA2NjM3NDgxNDU4OTI2ODAwOQ.Gn50Bz.-xewqMgFgsEdDq9jiNyT46OoOsnhEedGH-WYSU
ENTRYPOINT [ "./spectre" ]

# #cacher
# FROM lukemathwalker/cargo-chef as cacher
# WORKDIR /app
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --recipe-path recipe.json

# #builder
# FROM rust:latest as builder
# WORKDIR /app

# COPY migrations.sql /app/migrations.sql
# COPY --from=cacher /app/target target
# COPY --from=cacher /usr/local/cargo /usr/local/cargo
# COPY . .

# RUN cargo build

# #runtime
# FROM debian:buster-slim AS runtime
# WORKDIR /app
# COPY --from=builder /app/target/debug/spectre spectre
# COPY postgres.conf /etc/postgresql/postgresql.conf
# RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*

# ENV DISCORD_TOKEN=MTA2NjM3NDgxNDU4OTI2ODAwOQ.Gn50Bz.-xewqMgFgsEdDq9jiNyT46OoOsnhEedGH-WYSU
# ENTRYPOINT [ "./spectre" ]