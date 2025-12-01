# Telegram bot with Rust + Teloxide
### Task
Create some UI screens with captures and inline buttons. Make movement inside one message

## Установка

```bsah
git clone https://github.com/Avdushin/tg-captures
cd tg-captures
cp -rf .env.example .env # Токен бота сюда добавить в `TELOXIDE_TOKEN=`
rustup install nightly
rustup default nightly

cargo fetch
cargo run
```

## Demo
![start](./assets/docs/start.png) \
![help](./assets/docs/help.png) \
![menu](./assets/docs/menu.png) 