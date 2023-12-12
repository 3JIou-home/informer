use crate::message_actions::send_message;
use crate::message_data::{process_item, TempiKap, VecGoCrowd, VecIkap};
use chrono::{DateTime, Duration, Utc};
use log::{error, info};
use redis::{Client as RedisClient, Commands};
use reqwest::Client;
use scraper::{Html, Selector};
use teloxide::types::ChatId;
use teloxide::Bot;

pub async fn fetch_and_process_data(
    client: &Client,
    redis_client: &RedisClient,
    bot: &Bot,
    chat_id: ChatId,
) {
    let mut conn = redis_client.get_connection().unwrap();
    let res_gocrowd = match client
        .get("https://gocrowd.io/api/v2/offerings")
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("Error fetching data from GoCrowd: {:?}", e);
            return;
        }
    };
    let res_ikap = match client
        .get("https://ikapitalist.kz/")
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("Error fetching data from ikapitalist: {:?}", e);
            return;
        }
    };
    let html_content = res_ikap.text().await.unwrap();
    let document = Html::parse_document(&html_content);
    let selector_cart = Selector::parse(".info-item_full").unwrap();
    let selector_goal =
        Selector::parse(".card__num:not(.card__num_rate) .card__num-value").unwrap();
    let selector_rate = Selector::parse(".card__num_rate .card__num-value").unwrap();

    let selector_title = Selector::parse(".card__title").unwrap();
    let selector_img = Selector::parse(".card__img").unwrap();
    let selector_progress = Selector::parse(".card__progress-bar").unwrap();
    let mut i_kap: VecIkap = vec![];

    for cart in document.select(&selector_cart) {
        let mut temp: TempiKap = Default::default();

        temp.title = cart
            .select(&selector_title)
            .next()
            .map(|element| {
                element
                    .inner_html()
                    .chars()
                    .filter(|&c| !c.is_whitespace())
                    .collect()
            })
            .unwrap_or_default();

        temp.img = cart
            .select(&selector_img)
            .next()
            .and_then(|element| element.value().attr("src"))
            .unwrap_or_default()
            .to_string();

        temp.target.goal_value = cart
            .select(&selector_goal)
            .next()
            .map(|element| {
                element
                    .inner_html()
                    .chars()
                    .filter(|&c| !c.is_whitespace())
                    .collect()
            })
            .unwrap_or_default();

        temp.target.rate_value = cart
            .select(&selector_rate)
            .next()
            .map(|element| {
                element
                    .inner_html()
                    .chars()
                    .filter(|&c| !c.is_whitespace())
                    .collect()
            })
            .unwrap_or_default();

        temp.progress = cart
            .select(&selector_progress)
            .next()
            .map(|element| {
                element
                    .inner_html()
                    .chars()
                    .filter(|&c| !c.is_whitespace())
                    .collect()
            })
            .unwrap_or_default();

        i_kap.push(temp);
    }
    for item in i_kap.iter() {
        match conn.exists::<&String, bool>(&item.title) {
            Ok(check_exist_in_db) if check_exist_in_db => {
                info!("{} exists", item.title);
            }
            Ok(_) => {
                let fields = [
                    ("img", &item.img),
                    ("progress", &item.progress),
                    ("goal_value", &item.target.goal_value),
                    ("rate_value", &item.target.rate_value),
                ];
                conn.hset_multiple::<_, _, _, ()>(&item.title, &fields)
                    .expect("Failed to set hash");

                send_message(&bot, chat_id, item).await;
            }
            Err(e) => {
                error!("Error checking database: {:?}", e);
            }
        }
        info!("{:?}", item);
    }
    let mut answer_gocrowd =
        serde_json::from_str::<VecGoCrowd>(res_gocrowd.text().await.unwrap().as_str()).unwrap();

    for item in answer_gocrowd.iter_mut() {
        item.update_current_reserved_amount_str();
        item.update_interest_rate_str();
        item.update_max_target_str();
        let start_date = match DateTime::parse_from_rfc3339(&item.start_date) {
            Ok(date) => date,
            Err(e) => {
                error!("Error parsing date: {:?}", e);
                continue;
            }
        };

        let now = Utc::now().with_timezone(start_date.offset());

        match conn.exists::<&str, bool>(&item.id.to_string()) {
            Ok(check_exist_in_db) if check_exist_in_db => {
                info!("{} exists", item.id);
                continue;
            }
            Ok(_) => {}
            Err(e) => {
                error!("Error checking database: {:?}", e);
                continue;
            }
        }
        let check_exist_in_db: bool = conn.exists(item.id.to_string()).unwrap();
        if check_exist_in_db {
            info!("{} exists", item.id.to_string());
        } else if now - start_date < Duration::hours(1) {
            send_message(&bot, chat_id, item).await;
            info!("{} does not exist", item.id.to_string());
        }
        let fields: Vec<(&str, String)> = process_item(item);
        if let Err(e) = conn.hset_multiple::<_, _, _, ()>(&item.id.to_string(), &fields) {
            error!("Failed to set hash: {:?}", e);
        }
    }
}
