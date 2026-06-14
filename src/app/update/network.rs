use iced::Task;
use iced::widget::scrollable;
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::network::{ServerEvent, ClientCommand};

pub fn poll(model: &mut Model) -> Task<UiMessage> {
    while let Ok(ev) = model.rx.try_recv() {
        return Task::perform(async move { ev }, UiMessage::Network);
    }
    Task::none()
}

pub fn handle(model: &mut Model, ev: ServerEvent) -> Task<UiMessage> {
    use ServerEvent::*;
    match ev {
        AuthOk(sid) => {
            model.state.session_id = Some(sid.clone());
            
            if let crate::domain::state::Screen::AuthForm { username, nickname, .. } = &model.state.screen {
                let mut full_username = username.clone();
                if !full_username.contains('@') {
                    let domain = model.state.server_address
                        .strip_prefix("ws://")
                        .unwrap_or(&model.state.server_address)
                        .strip_suffix("/ws")
                        .unwrap_or(&model.state.server_address)
                        .to_string();
                    full_username = format!("{}@{}", full_username, domain);
                }
                
                model.state.username = Some(full_username.clone());
                model.state.nickname = Some(nickname.clone());
                
                let db = model.db.clone();
                let u = full_username.clone();
                let s = sid.clone();
                tokio::spawn(async move {
                    db.save_local_session(&u, &s).await;
                });
            } else if let Some((saved_user, _)) = &model.state.saved_session {
                model.state.username = Some(saved_user.clone());
            }
            
            model.state.screen = crate::domain::state::Screen::ChatList;
            model.state.status = "Успешная авторизация".into();
            Task::none()
        }
        AuthErr(e) => {
            model.state.status = format!("Ошибка: {}", e);
            if model.state.saved_session.is_some() {
                model.state.saved_session = None;
                
                let db = model.db.clone();
                tokio::spawn(async move {
                    db.clear_local_session().await;
                });
                
                model.state.screen = crate::domain::state::Screen::MainMenu { selection: 0 };
                model.state.username = None;
                model.state.session_id = None;
            }
            Task::none()
        }
        ServerEvent::ProfileReceived(user) => {
            model.state.cache_profile(&user);
            return Task::done(UiMessage::ProfileReceived(user));
        }
        ServerEvent::ProfileUpdated => {
            if let Some(ref username) = model.state.username.clone() {
                let _ = model.cmd_tx.send(ClientCommand::RequestProfile(username.clone()));
            }
            Task::none()
        }
        NewMessage(mut msg) => {
            msg.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            model.state.messages.push(msg.clone());
            
            let db = model.db.clone();
            let m = msg.clone();
            tokio::spawn(async move {
                let _ = db.save_message(&m).await;
            });

            let my_username = model.state.username.as_deref().unwrap_or("");
            let mut should_update = false;

            if msg.from != my_username && !model.state.chats.contains(&msg.from) {
                model.state.chats.push(msg.from.clone());
                model.state.chats.sort();
                should_update = true;
            }

            if !model.state.peer_keys.contains_key(&msg.from) {
                let _ = model.cmd_tx.send(ClientCommand::FetchPeerKeys(msg.from.clone()));
            }

            if !model.state.nickname_cache.contains_key(&msg.from) {
                let _ = model.cmd_tx.send(ClientCommand::RequestProfile(msg.from.clone()));
            }

            let sender = msg.from.clone();

            if sender != my_username {
                let is_active_chat = matches!(&model.state.screen, crate::domain::state::Screen::ChatView { target, .. } if target == &sender);

                if !is_active_chat {
                    play_notification_sound();

                    let sender_clone = sender.clone();
                    tokio::task::spawn_blocking(move || {
                        let _ = notify_rust::Notification::new()
                            .appname("AnonPeer")
                            .summary(&format!("Новое сообщение от {}", sender_clone))
                            .body("Откройте AnonPeer, чтобы прочитать")
                            .icon("dialog-information")
                            .show();
                    });
                }
            }

            if should_update {
                Task::batch(vec![
                    scrollable::scroll_to(model.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX }),
                    Task::done(UiMessage::Tick)
                ])
            } else {
                scrollable::scroll_to(model.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX })
            }
        }
        Disconnect => {
            model.state.status = "Соединение разорвано".into();
            Task::none()
        }
        PeerKeys { target, ed_public, x25519_public } => {
            model.state.cache_peer_keys(target.clone(), ed_public.clone(), x25519_public.clone());
            Task::done(UiMessage::KeysReceived { target, ed_public, x25519_public })
        }
        SearchResults(matches) => {
            model.search_matches = matches;
            Task::none()
        }
    }
}

fn play_notification_sound() {
    std::thread::spawn(|| {
        let sound_bytes: &[u8] = include_bytes!("../../../assets/sounds/notification.wav");
        if let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() {
            if let Ok(sink) = rodio::Sink::try_new(&stream_handle) {
                let cursor = std::io::Cursor::new(sound_bytes);
                if let Ok(source) = rodio::Decoder::new(cursor) {
                    sink.append(source);
                    sink.sleep_until_end();
                }
            }
        }
    });
}