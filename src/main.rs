use dotenvy::dotenv;
use env_logger;
use log::{info, error};

use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InputFile, InputMedia, InputMediaPhoto,
        ChatId, MessageId, CallbackQuery,
    },
};
use teloxide::dptree;

// Экран, который сейчас показываем
#[derive(Clone, Copy)]
enum Screen {
    Start,
    Help,
    Menu,
}

#[tokio::main]
async fn main() {
    // Подхватываем .env
    dotenv().ok();
    env_logger::init();
    info!("Starting bot...");

    // Токен берётся из TELOXIDE_TOKEN
    let bot = Bot::from_env();

    // dptree-handler: обрабатываем и сообщения, и callback-запросы
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback_query));

    Dispatcher::builder(bot, handler)
        .build()
        .dispatch()
        .await;
}

// ---------------- ЛОГИКА СООБЩЕНИЙ ----------------

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
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
                bot.send_message(msg.chat.id, "Не знаю такую команду. Попробуй /start, /help или /menu.")
                    .await?;
            }
        }
    }
    Ok(())
}

// ---------------- ЛОГИКА CALLBACK-КНОПОК ----------------

async fn handle_callback_query(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        let screen = match data.as_str() {
            "start" => Screen::Start,
            "help"  => Screen::Help,
            "menu"  => Screen::Menu,
            _       => Screen::Start,
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

async fn send_screen(
    bot: Bot,
    chat_id: ChatId,
    screen: Screen,
    message_id: Option<MessageId>,
) {
    let (text, image_path) = screen_content(screen);
    let keyboard = screen_keyboard();

    // Загружаем картинку (локальный файл)
    let photo = InputFile::file(image_path);

    if let Some(msg_id) = message_id {
        // Редактируем медиа и подпись в существующем сообщении
        let media = InputMedia::Photo(
            InputMediaPhoto::new(photo).caption(text.to_string())
        );

        if let Err(err) = bot
            .edit_message_media(chat_id, msg_id, media)
            .reply_markup(keyboard)
            .await
        {
            error!("Error editing media: {err:?}");
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

// Текст и путь к картинке для каждого экрана
fn screen_content(screen: Screen) -> (&'static str, &'static str) {
    match screen {
        Screen::Start => (
            "Привет! 👋 Я Rust Telegram бот.\n\nИспользуй кнопки ниже, чтобы переключаться между экранами.",
            "assets/start.jpg",
        ),
        Screen::Help => (
            "Помощь 🆘\n\nДоступные команды:\n/start - приветствие\n/help - список команд\n/menu - меню с кнопками\n\nЛистай экраны кнопками ниже.",
            "assets/help.webp",
        ),
        Screen::Menu => (
            "Меню 📋\n\nЗдесь может быть ваше основное приложение, список действий и т.п.",
            "assets/menu.jpg",
        ),
    }
}

// Одна и та же навигация на всех экранах
fn screen_keyboard() -> InlineKeyboardMarkup {
    let row = vec![
        InlineKeyboardButton::callback("🏠 Старт", "start"),
        InlineKeyboardButton::callback("❓ Help", "help"),
        InlineKeyboardButton::callback("📋 Меню", "menu"),
    ];

    InlineKeyboardMarkup::new(vec![row])
}
