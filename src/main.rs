mod data_fetcher;
mod message_actions;
mod message_data;

use data_fetcher::fetch_and_process_data;
use redis;
use reqwest;
use teloxide::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    redis: String,

    #[arg(long)]
    tg_bot_token: String,

    #[arg(long)]
    tg_chat_id: i64,

    #[arg(short, long, default_value_t = 300)]
    retry: u64,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let bot = Bot::new(args.tg_bot_token);
    let chat_id = ChatId(args.tg_chat_id);
    let redis_client = redis::Client::open(format!("redis://{}", args.redis)).unwrap();

    let client = reqwest::Client::new();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(args.retry));
    loop {
        interval.tick().await;
        fetch_and_process_data(&client, &redis_client, &bot, chat_id).await;
    }
}
