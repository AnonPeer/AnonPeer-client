# AnonPeer-client
## 📋 Описание

AnonPeer-client — это кроссплатформенное настольное приложение на языке Rust, предоставляющее безопасный анонимный мессенджер с графическим интерфейсом. Приложение поддерживает шифрование сообщений, локальное хранение данных и работу в реальном времени через WebSocket.

## ✨ Возможности

- 🔐 Сквозное шифрование с использованием ключей Ed25519/X25519
- 🖥️ Нативный GUI на базе фреймворка Iced 0.13
- 🗄️ Локальная база данных SQLite для хранения истории сообщений
- 🔄 Асинхронная связь с сервером через WebSocket (tokio-tungstenite)
- 🧵 Полностью асинхронная архитектура на Tokio
- 📊 Структурированное логирование через `tracing`
- 🚩 Поддержка федераций

## Структура проекта
```
├── assets
│   ├── fonts
│   │   ├── NotoColorEmoji.ttf
│   │   └── NotoSansSymbols-VariableFont_wght.ttf
│   └── sounds
│       └── notification.wav
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── app
    │   ├── component
    │   │   ├── input.rs
    │   │   ├── message.rs
    │   │   ├── mod.rs
    │   │   └── sidebar.rs
    │   ├── message.rs
    │   ├── model.rs
    │   ├── mod.rs
    │   ├── theme
    │   │   ├── colors.rs
    │   │   ├── mod.rs
    │   │   └── styles.rs
    │   └── ui
    │       ├── layout.rs
    │       ├── mod.rs
    │       └── screens
    │           ├── mod.rs
    │           └── windows.rs
    ├── ico.ico
    ├── main.rs
    ├── network.rs
    └── state.rs
```
## 🚀 Установка

### Требования

- Инструментарий Rust (версия 1.70 или новее)
- Менеджер пакетов Cargo
- Системные инструменты сборки (gcc, make и т.д.)

### Сборка из исходников

```bash
git clone https://github.com/AnonPeer/AnonPeer-client.git
git clone https://github.com/AnonPeer/AnonPeer-shared.git
mv AnonPeer-client client
mv AnonPeer-shared shared
cargo build --release
```
## Запуск

### Windows
```
cargo run --release
```
### Linux
```
cargo run --release
```
## 📄 Лицензия
 Проект распространяется под лицензией MIT
