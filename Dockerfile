FROM rust:1.74.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/informer /
ENV REDIS_URL=""
ENV TG_BOT_TOKEN=""
ENV TG_CHAT_ID=""
ENV RETRY="300"
CMD ["informer --redis $REDIS_URL --tg-bot-token $TG_BOT_TOKEN --tg-chat-id $TG_CHAT_ID --retry $RETRY"]
