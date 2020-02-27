FROM rust:1.41.0
WORKDIR /app
RUN apt update && apt upgrade -y
RUN apt install postgresql-client -y
RUN cargo install diesel_cli --no-default-features --features postgres
RUN curl -f https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh -o wait-for-it.sh && chmod +x wait-for-it.sh
COPY . .
RUN cargo install --path .
CMD ["./docker-entry.sh"]
