FROM rust:buster AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim as runner
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/dso_dojo_slow_postgres .
COPY --from=builder /app/files ./files
ENV ROCKET_DATABASES='{postgres={url="postgres://postgres:plaintext@localhost:5432/postgres"}}'
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./dso_dojo_slow_postgres"]
