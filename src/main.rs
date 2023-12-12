mod data_fetcher;
mod message_actions;
mod message_data;

use clap::Parser;
use data_fetcher::fetch_and_process_data;
use redis;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use teloxide::prelude::*;

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

    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36").unwrap()
    );
    headers.insert(
        "Accept",
        HeaderValue::from_str("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8").unwrap()
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_str("en-US,en;q=0.5").unwrap()
    );
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .default_headers(headers)
        .build().unwrap();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(args.retry));
    loop {
        interval.tick().await;
        fetch_and_process_data(&client, &redis_client, &bot, chat_id).await;
    }
}
