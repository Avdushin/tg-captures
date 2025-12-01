use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::types::{Callbacks, Screen};

// Текст и путь к картинке для каждого экрана
pub fn screen_content(screen: Screen) -> (&'static str, &'static str) {
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
pub fn screen_keyboard() -> InlineKeyboardMarkup {
    let row = vec![
        InlineKeyboardButton::callback("🏠 Старт", Callbacks::Start.to_string()),
        InlineKeyboardButton::callback("❓ Help", Callbacks::Help.to_string()),
        InlineKeyboardButton::callback("📋 Меню", Callbacks::Menu.to_string()),
    ];

    InlineKeyboardMarkup::new(vec![row])
}
