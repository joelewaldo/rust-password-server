FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

COPY . .

WORKDIR /app/docker

RUN cp .env.example .env

RUN docker-compose up -d

WORKDIR /app

RUN cp .env.example .env

CMD ["sh", "-c", "cargo test && docker-compose down"]