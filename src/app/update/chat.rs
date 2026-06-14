use iced::Task;
use iced::widget::scrollable;
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::network::ClientCommand;
use shared::crypto::encrypt_sign;

pub fn select(model: &mut Model, target: String) -> Task<UiMessage> {
    if !model.state.chats.contains(&target) && model.state.username.as_ref().map_or(true, |u| u != &target) {
        model.state.chats.push(target.clone());
        model.state.chats.sort();
    }
    model.selected_chat = Some(target.clone());
    model.state.screen = crate::domain::state::Screen::ChatView { target: target.clone(), input: String::new() };
    model.state.status.clear();

    if !model.state.peer_keys.contains_key(&target) {
        let _ = model.cmd_tx.send(ClientCommand::FetchPeerKeys(target.clone()));
    }
    if !model.state.nickname_cache.contains_key(&target) {
        let _ = model.cmd_tx.send(ClientCommand::RequestProfile(target.clone()));
    }

    scrollable::scroll_to(model.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX })
}

pub fn input_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    if let crate::domain::state::Screen::ChatView { input, .. } = &mut model.state.screen {
        *input = v;
    }
    Task::none()
}

pub fn open_new(model: &mut Model) -> Task<UiMessage> {
    model.state.screen = crate::domain::state::Screen::NewChat { input: String::new() };
    model.state.status.clear();
    Task::none()
}

pub fn new_input_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    if let crate::domain::state::Screen::NewChat { input } = &mut model.state.screen {
        *input = v;
    }
    Task::none()
}

pub fn new_submit(model: &mut Model) -> Task<UiMessage> {
    if let crate::domain::state::Screen::NewChat { input } = &model.state.screen {
        let target = input.trim().to_string();
        if !target.is_empty() {
            if !model.state.chats.contains(&target) && model.state.username.as_ref().map_or(true, |u| u != &target) {
                model.state.chats.push(target.clone());
                model.state.chats.sort();
            }
            model.selected_chat = Some(target.clone());
            model.state.screen = crate::domain::state::Screen::ChatView {
                target: target.clone(),
                input: String::new(),
            };
            if !model.state.peer_keys.contains_key(&target) {
                let _ = model.cmd_tx.send(ClientCommand::FetchPeerKeys(target));
            }
        }
    }
    Task::none()
}

pub fn new_cancel(model: &mut Model) -> Task<UiMessage> {
    model.state.screen = crate::domain::state::Screen::ChatList;
    model.state.status.clear();
    Task::none()
}

pub fn send(model: &mut Model) -> Task<UiMessage> {
    let input_text = if let crate::domain::state::Screen::ChatView { input, .. } = &model.state.screen {
        input.clone()
    } else {
        String::new()
    };

    if !input_text.trim().is_empty() {
        send_message_content(model, shared::protocol::MessageContent::Text(input_text));
    }
    
    scrollable::scroll_to(model.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX })
}

pub fn send_image(model: &mut Model, mime: String, data: String) -> Task<UiMessage> {
    send_message_content(model, shared::protocol::MessageContent::Image { mime_type: mime, base64_data: data });
    scrollable::scroll_to(model.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX })
}

fn send_message_content(model: &mut Model, content: shared::protocol::MessageContent) {
    if let crate::domain::state::Screen::ChatView { target, .. } = &model.state.screen {
        let target = target.clone();
        
        if let Some(user) = &model.state.username {
            if let Ok(chat_key) = model.state.get_chat_key(&target) {
                let pt = match serde_json::to_vec(&content) {
                    Ok(v) => v,
                    Err(_) => {
                        model.state.status = "Ошибка сериализации".into();
                        return;
                    }
                };

                if let Ok((ct, nonce, sig, _eph_public)) = shared::crypto::encrypt_sign(&pt, &chat_key, &model.state.ed_secret) {
                    let msg = shared::protocol::AppMessage {
                        id: uuid::Uuid::new_v4(),
                        from: user.clone(),
                        to: target.clone(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        ciphertext: ct,
                        nonce,
                        signature: sig,
                        salt: vec![],
                    };

                    model.state.messages.push(msg.clone());
                    
                    let db = model.db.clone();
                    let m = msg.clone();
                    tokio::spawn(async move {
                        let _ = db.save_message(&m).await;
                    });

                    let _ = model.cmd_tx.send(ClientCommand::Send(msg));

                    if let shared::protocol::MessageContent::Text(_) = content {
                        if let crate::domain::state::Screen::ChatView { input, .. } = &mut model.state.screen {
                            *input = String::new();
                        }
                    }
                } else {
                    model.state.status = "Ошибка шифрования".into();
                }
            } else {
                model.state.status = "Нет ключей собеседника".into();
                let _ = model.cmd_tx.send(ClientCommand::FetchPeerKeys(target.clone()));
            }
        }
    }
}

pub fn fetch_keys(model: &mut Model, t: String) -> Task<UiMessage> {
    let _ = model.cmd_tx.send(ClientCommand::FetchPeerKeys(t));
    Task::none()
}

pub fn keys_received(model: &mut Model, target: String, ed_public: Vec<u8>, x25519_public: Vec<u8>) -> Task<UiMessage> {
    model.state.cache_peer_keys(target, ed_public, x25519_public);
    Task::none()
}

pub fn search_input_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    model.search_query = v.clone();
    if v.len() >= 2 {
        let _ = model.cmd_tx.send(ClientCommand::SearchPrefix(v));
    } else {
        model.search_matches.clear();
    }
    Task::none()
}

pub fn search_submit(model: &mut Model) -> Task<UiMessage> {
    if !model.search_query.trim().is_empty() {
        let _ = model.cmd_tx.send(ClientCommand::SearchPrefix(model.search_query.clone()));
    }
    Task::none()
}

pub fn search_results(model: &mut Model, matches: Vec<shared::protocol::UserInfo>) -> Task<UiMessage> {
    model.search_matches = matches;
    Task::none()
}

pub fn search_result_selected(model: &mut Model, name: String) -> Task<UiMessage> {
    model.search_query.clear();
    model.search_matches.clear();
    Task::done(UiMessage::ChatSelected(name))
}

pub fn search_result(model: &mut Model, username: String, exists: bool) -> Task<UiMessage> {
    if exists {
        model.search_query.clear();
        model.search_matches.clear();
        return Task::done(UiMessage::ChatSelected(username));
    }
    Task::none()
}