use iced::widget::{column, container, row, text, Space, Column};
use iced::{Alignment, Element, Length};
use shared::protocol::AppMessage;
use shared::crypto::decrypt_verify;
use crate::app::message::UiMessage;
use crate::state::AppState;
use crate::app::theme::{colors, styles};
use chrono::TimeZone;

pub fn view_bubble<'a>(msg: &'a AppMessage, state: &'a AppState, my_username: &str) -> Element<'a, UiMessage> {
    let is_my = msg.from == my_username;
    let time_str = chrono::Local.timestamp_opt(msg.timestamp as i64, 0)
        .single()
        .map(|d| d.format("%H:%M").to_string())
        .unwrap_or_else(|| "??:??".to_string());
    let chat_key = state.get_chat_key(if is_my { &msg.to } else { &msg.from }).ok();
    let sender_pub = state.get_sender_public_key(&msg.from);
    
    let raw_content = if let (Some(key), Some(pub_key)) = (chat_key, sender_pub) {
        match decrypt_verify(&msg.ciphertext, &msg.nonce, &msg.signature, &key, pub_key) {
            Ok(plain) => String::from_utf8_lossy(&plain).into_owned(),
            Err(_) => "🔒 Ошибка расшифрования".to_string(),
        }
    } else { "⏳ Ожидание ключей...".to_string() };

    let mut msg_block = Column::new().spacing(3).height(Length::Shrink);
    if !is_my {
        msg_block = msg_block.push(text(&msg.from).size(11).style(|_| iced::widget::text::Style { color: Some(colors::ACCENT) }));
    }
    msg_block = msg_block.push(
        row![text(raw_content).size(14), Space::with_width(12), text(time_str).size(10).style(styles::muted_text())].align_y(Alignment::End)
    );
    
    let bubble = container(msg_block).padding([8, 14]).style(styles::msg_bubble(is_my));
    column![bubble].width(Length::Fill).align_x(if is_my { Alignment::End } else { Alignment::Start }).into()
}