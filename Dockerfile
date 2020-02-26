FROM rust:1.41.0
WORKDIR /app
RUN apt update && apt upgrade -y
RUN apt install postgresql-client -y
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .
RUN cargo install --path .
CMD ["./docker-entry.sh"]
