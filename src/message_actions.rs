use crate::message_data::{GoCrowd, Ikap, MessageData};
use std::any::TypeId;
use teloxide::payloads::SendPhotoSetters;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::InputFile;
use teloxide::Bot;

pub async fn send_message<T: MessageData + Sync>(bot: &Bot, chat_id: ChatId, item: &T) {
    let logo_url = item.logo_url();
    let caption = create_caption(item);

    if let Ok(logo_url) = logo_url.parse() {
        bot.send_photo(chat_id, InputFile::url(logo_url))
            .caption(caption)
            .await
            .expect("Не удалось отправить сообщение с фото");
    } else {
        bot.send_message(chat_id, caption)
            .await
            .expect("Не удалось отправить сообщение без фото");
    }
}

pub fn create_caption<T: MessageData>(item: &T) -> String {
    let type_name = if item.type_id() == TypeId::of::<Ikap>() {
        "ikapitalist"
    } else if item.type_id() == TypeId::of::<GoCrowd>() {
        "gocrowd"
    } else {
        "неизвестный"
    };

    format!(
        "На {} новое предложение: {}\nЦель собрать: {}\nПроцент сбора: {}\nПроцент по инвестиции: {}",
        type_name,
        item.title(),
        item.goal(),
        item.progress(),
        item.rate(),
    )
}
