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


## Структура проекта
```
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
│   ├── ui
│   │   ├── layout.rs
│   │   ├── mod.rs
│   │   └── screens
│   │       ├── mod.rs
│   │       └── windows.rs
│   └── windows.rs
├── ico.png
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
cd AnonPeer-client
cargo build --release
```
## Запуск

### Windows
 $env:ANON_SERVER="ws://144.31.215.157:3000/ws"
 cargo run --release

### Linux
 ANON_SERVER="ws://144.31.215.157:3000/ws"
 cargo run --release

## 📄 Лицензия
 Проект распространяется под лицензией MIT
