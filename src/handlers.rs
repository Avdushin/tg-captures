use crate::types::Screen;
use crate::ui::{screen_content, screen_keyboard};
use log::error;
use teloxide::{
    prelude::*,
    types::{CallbackQuery, ChatId, InputFile, InputMedia, InputMediaPhoto, MessageId},
};

// ---------------- ЛОГИКА СООБЩЕНИЙ ----------------
pub async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match text {
            "/start" => {
                send_screen(bot, msg.chat.id, Screen::Start, None).await;
            }
            "/help" => {
                send_screen(bot, msg.chat.id, Screen::Help, None).await;
            }
            "/menu" => {
                send_screen(bot, msg.chat.id, Screen::Menu, None).await;
            }
            _ => {
                bot.send_message(
                    msg.chat.id,
                    "Не знаю такую команду. Попробуй /start, /help или /menu.",
                )
                .await?;
            }
        }
    }
    Ok(())
}

// ---------------- ЛОГИКА CALLBACK-КНОПОК ----------------

pub async fn handle_callback_query(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        let screen = match data.as_str() {
            "start" => Screen::Start,
            "help" => Screen::Help,
            "menu" => Screen::Menu,
            _ => Screen::Start,
        };

        if let Some(msg) = q.message {
            // редактируем существующее сообщение
            send_screen(bot.clone(), msg.chat().id, screen, Some(msg.id())).await;
        }

        // Ответ на callback, чтобы пропали «часики»
        bot.answer_callback_query(q.id).await?;
    }

    Ok(())
}

// ---------------- ОТРИСОВКА ЭКРАНА ----------------
async fn send_screen(bot: Bot, chat_id: ChatId, screen: Screen, message_id: Option<MessageId>) {
    let (text, image_path) = screen_content(screen);
    let keyboard = screen_keyboard();

    // Загружаем картинку (локальный файл)
    let photo = InputFile::file(image_path);

    if let Some(msg_id) = message_id {
        // Редактируем медиа и подпись в существующем сообщении
        let media = InputMedia::Photo(InputMediaPhoto::new(photo).caption(text.to_string()));

        if let Err(_err) = bot
            .edit_message_media(chat_id, msg_id, media)
            .reply_markup(keyboard)
            .await
        {
            // eprint!("Error editing media: {err:?}");
            return
        }
    } else {
        // Отправляем новое сообщение
        if let Err(err) = bot
            .send_photo(chat_id, photo)
            .caption(text.to_string())
            .reply_markup(keyboard)
            .await
        {
            error!("Error sending photo: {err:?}");
        }
    }
}
