FROM rust:1.41.0
WORKDIR /app
COPY . .
RUN apt update && apt upgrade -y
RUN apt install postgresql-client -y
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .
CMD ["postman"]
