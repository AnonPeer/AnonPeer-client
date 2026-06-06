use iced::widget::{column, container, row, text, Space, Column};
use iced::{Alignment, Element, Length};
use shared::protocol::{AppMessage, MessageContent};
use shared::crypto::decrypt_verify;
use crate::app::message::UiMessage;
use crate::app::model::Model;
use crate::app::theme::{colors, styles};
use chrono::TimeZone;
use base64::Engine;
use std::sync::Arc;

pub fn view_bubble<'a>(msg: &'a AppMessage, model: &'a Model, my_username: &str) -> Element<'a, UiMessage> {
    let state = &model.state;
    let is_my = msg.from == my_username;
    let time_str = chrono::Local.timestamp_opt(msg.timestamp as i64, 0)
        .single()
        .map(|d| d.format("%H:%M").to_string())
        .unwrap_or_else(|| "??:??".to_string());
    
    let chat_key = state.get_chat_key(if is_my { &msg.to } else { &msg.from }).ok();
    let sender_pub = state.get_sender_public_key(&msg.from);

    let mut msg_block = Column::new().spacing(3).height(Length::Shrink);
    if !is_my {
        msg_block = msg_block.push(text(&msg.from).size(11).style(|_| iced::widget::text::Style { color: Some(colors::ACCENT) }));
    }

    let cached_content = {
        let cache = model.decrypted_cache.read().unwrap();
        cache.get(&msg.id).cloned()
    };

    let content_arc = cached_content.unwrap_or_else(|| {
        let result = if let (Some(key), Some(pub_key)) = (chat_key, sender_pub) {
            if let Ok(plain) = decrypt_verify(&msg.ciphertext, &msg.nonce, &msg.signature, &key, pub_key) {
                match serde_json::from_slice::<MessageContent>(&plain) {
                    Ok(content) => Ok(content),
                    Err(_) => {
                        if let Ok(raw_text) = String::from_utf8(plain) {
                            Ok(MessageContent::Text(raw_text))
                        } else {
                            Err("json".to_string())
                        }
                    }
                }
            } else {
                Err("decrypt".to_string())
            }
        } else {
            return Arc::new(Err("keys".to_string()));
        };
        
        let arc_result = Arc::new(result);
        if !matches!(arc_result.as_ref(), Err(e) if e == "keys") {
            if let Ok(mut cache) = model.decrypted_cache.write() {
                cache.insert(msg.id, arc_result.clone());
            }
        }
        arc_result
    });

    match content_arc.as_ref() {
        Ok(MessageContent::Text(t)) => {
            msg_block = msg_block.push(
                row![text(t.clone()).size(14), Space::with_width(12), text(time_str).size(10).style(styles::muted_text())].align_y(Alignment::End)
            );
        }

        Ok(MessageContent::Image { mime_type: _, base64_data }) => {
            let handle = {
                let cache = model.image_cache.read().unwrap();
                cache.get(&msg.id).cloned()
            };

            let handle = handle.unwrap_or_else(|| {
                if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(base64_data) {
                    let new_handle = iced::widget::image::Handle::from_bytes(decoded);
                    if let Ok(mut cache) = model.image_cache.write() {
                        cache.insert(msg.id, new_handle.clone());
                    }
                    new_handle
                } else {
                    iced::widget::image::Handle::from_bytes(vec![])
                }
            });

            let img = iced::widget::Image::new(handle.clone()).width(Length::Fixed(200.0)); 
            
            let clickable_img = iced::widget::button(img)
                .style(|_, _| iced::widget::button::Style {
                    background: Some(iced::Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(UiMessage::ExpandImage(msg.id)); 

            msg_block = msg_block.push(clickable_img);
            msg_block = msg_block.push(text(time_str).size(10).style(styles::muted_text()).align_x(Alignment::End));
        }

        Err(e) => {
            let err_text = match e.as_str() {
                "keys" => "⏳ Ожидание ключей...",
                "decrypt" => "🔒 Ошибка расшифрования",
                "json" => "🔒 Ошибка формата сообщения",
                _ => "🔒 Ошибка",
            };
            msg_block = msg_block.push(text(err_text).size(14));
        }
    }

    let bubble = container(msg_block).padding([8, 14]).style(styles::msg_bubble(is_my));
    column![bubble].width(Length::Fill).align_x(if is_my { Alignment::End } else { Alignment::Start }).into()
}