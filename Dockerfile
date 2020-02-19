FROM rust:1.41.0
WORKDIR /app
COPY . .
RUN apt-get install libsqlite3-dev
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel migration run
RUN cargo install --path .
CMD ["postman"]
