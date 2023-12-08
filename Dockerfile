FROM rust:1.74.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build-env /app/target/release/informer /
ENV REDIS_URL=""
ENV TG_BOT_TOKEN=""
ENV TG_CHAT_ID=""
ENV RETRY="300"
CMD ["sh", "-c", "/informer --redis $REDIS_URL --tg-bot-token $TG_BOT_TOKEN --tg-chat-id $TG_CHAT_ID --retry $RETRY"]
