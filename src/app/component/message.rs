use iced::widget::{column, container, row, text, Space, Column};
use iced::{Alignment, Element, Length};
use shared::protocol::{AppMessage, MessageContent};
use shared::crypto::decrypt_verify;
use crate::app::message::UiMessage;
use crate::state::AppState;
use crate::app::theme::{colors, styles};
use chrono::TimeZone;
use base64::Engine; 

pub fn view_bubble<'a>(msg: &'a AppMessage, state: &'a AppState, my_username: &str) -> Element<'a, UiMessage> {
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

    if let (Some(key), Some(pub_key)) = (chat_key, sender_pub) {
        if let Ok(plain) = decrypt_verify(&msg.ciphertext, &msg.nonce, &msg.signature, &key, pub_key) {
            
            if let Ok(content) = serde_json::from_slice::<MessageContent>(&plain) {
                match content {
                    MessageContent::Text(t) => {
                        msg_block = msg_block.push(
                            row![text(t).size(14), Space::with_width(12), text(time_str).size(10).style(styles::muted_text())].align_y(Alignment::End)
                        );
                    }
                    MessageContent::Image { mime_type: _, base64_data } => {
                        if let Ok(img_bytes) = base64::engine::general_purpose::STANDARD.decode(&base64_data) {
                            let img = iced::widget::Image::new(iced::widget::image::Handle::from_bytes(img_bytes))
                                .width(Length::Fixed(200.0)); 
                            msg_block = msg_block.push(img);
                            msg_block = msg_block.push(text(time_str).size(10).style(styles::muted_text()).align_x(Alignment::End));
                        } else {
                            msg_block = msg_block.push(text("🔒 Ошибка декодирования изображения").size(14));
                        }
                    }
                }
            } else {
                let raw_text = String::from_utf8_lossy(&plain);
                msg_block = msg_block.push(
                    row![text(raw_text.into_owned()).size(14), Space::with_width(12), text(time_str).size(10).style(styles::muted_text())].align_y(Alignment::End)
                );
            }
        } else {
            msg_block = msg_block.push(text("🔒 Ошибка расшифрования").size(14));
        }
    } else {
        msg_block = msg_block.push(text("⏳ Ожидание ключей...").size(14));
    }

    let bubble = container(msg_block).padding([8, 14]).style(styles::msg_bubble(is_my));
    column![bubble].width(Length::Fill).align_x(if is_my { Alignment::End } else { Alignment::Start }).into()
}