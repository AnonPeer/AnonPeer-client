use iced::widget::{button, column, container, row, text, text_input, Column, Scrollable, Space, Image};
use iced::{Alignment, Border, Element, Length};
use crate::app::message::UiMessage;
use crate::app::model::Model;
use crate::app::theme::{colors, styles};
use crate::NOTO_SANS;
use base64::Engine as _;

fn avatar_circle<'a>(avatar_b64: Option<&str>, label: &str, is_selected: bool) -> Element<'a, UiMessage> {
    let size = 36.0;
    if let Some(b64) = avatar_b64 {
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(b64) {
            let handle = iced::widget::image::Handle::from_bytes(bytes);
            return container(
                Image::new(handle)
                    .width(Length::Fixed(size))
                    .height(Length::Fixed(size))
            )
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(if is_selected { colors::ACCENT } else { colors::SURFACE })),
                text_color: Some(colors::TEXT_LIGHT),
                border: Border { radius: (size / 2.0).into(), width: 0.0, color: iced::Color::TRANSPARENT },
                ..Default::default()
            })
            .into();
        }
    }
    let first = label.chars().next().unwrap_or('?').to_uppercase().to_string();
    container(text(first).size(16).shaping(iced::widget::text::Shaping::Advanced))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .center_x(Length::Fixed(size))
        .center_y(Length::Fixed(size))
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(if is_selected { colors::ACCENT } else { colors::SURFACE })),
            text_color: Some(colors::TEXT_LIGHT),
            border: Border { radius: (size / 2.0).into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        })
        .into()
}

pub fn view<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let my_name = model.state.username.as_deref().unwrap_or("Аноним");
    let chats = &model.state.chats;
    let selected = model.selected_chat.as_ref();
    let mut col = Column::new().spacing(12).padding(16).height(Length::Fill);

    let my_avatar = model.state.username.as_ref()
        .and_then(|u| model.state.avatar_cache.get(u))
        .and_then(|a| a.as_deref());
    let my_nickname = model.state.nickname.as_deref().unwrap_or(my_name);

    col = col.push(row![
        avatar_circle(my_avatar, my_name, false),
        Space::with_width(10),
        column![
            text(my_nickname).size(16).shaping(iced::widget::text::Shaping::Advanced),
            text("в сети").size(12).style(styles::muted_text())
        ],
        Space::with_width(Length::Fill),
        button(text("✏").size(16).font(NOTO_SANS).shaping(iced::widget::text::Shaping::Advanced))
            .padding(6)
            .style(styles::surface_button())
            .on_press(UiMessage::OpenEditProfile)
    ].align_y(Alignment::Center));

    let search_input: Element<'a, UiMessage> = text_input(" 🔍  Поиск...", &model.search_query)
        .on_input(UiMessage::SearchInputChanged)
        .padding([10, 14])
        .size(14)
        .width(Length::Fill)
        .style(styles::modern_text_input())
        .into();
    col = col.push(search_input);

    if !model.search_matches.is_empty() {
        let mut dropdown = Column::new().spacing(2).padding(4);
        for user_info in &model.search_matches {
            let username_for_chat = if let Some(ref domain) = user_info.server_domain {
                format!("{}@{}", user_info.username, domain)
            } else {
                user_info.username.clone()
            };
            let display_name = if let Some(ref domain) = user_info.server_domain {
                format!("{} (@{}@{})", user_info.nickname, user_info.username, domain)
            } else {
                format!("{} (@{})", user_info.nickname, user_info.username)
            };
            let user_avatar = user_info.avatar_base64.as_deref();
            let row_content = row![
                avatar_circle(user_avatar, &user_info.username, false),
                Space::with_width(8),
                text(display_name).size(14).shaping(iced::widget::text::Shaping::Advanced)
            ].align_y(Alignment::Center);
            dropdown = dropdown.push(
                button(row_content)
                    .width(Length::Fill)
                    .padding([8, 12])
                    .style(styles::surface_button())
                    .on_press(UiMessage::SearchResultSelected(username_for_chat))
            );
        }
        col = col.push(
            container(dropdown)
                .width(Length::Fill)
                .style(styles::surface_container())
        );
    }

    col = col.push(Space::with_height(8));

    col = col.push(
        button(row![
            text("➕").size(18).font(NOTO_SANS).shaping(iced::widget::text::Shaping::Advanced),
            Space::with_width(8),
            text("Новый чат").size(14)
        ].align_y(Alignment::Center))
        .width(Length::Fill)
        .padding([10, 0])
        .style(styles::accent_button())
        .on_press(UiMessage::OpenNewChatScreen)
    );

    col = col.push(Space::with_height(8));
    col = col.push(text("Чаты").size(12).style(styles::muted_text()).shaping(iced::widget::text::Shaping::Advanced));

    let mut chats_list = Column::new().spacing(4);
    if chats.is_empty() {
        chats_list = chats_list.push(
            container(text("Нет активных чатов").size(13).style(styles::muted_text()))
                .padding(20).center_x(Length::Fill)
        );
    } else {
        for chat in chats {
            let is_selected = selected == Some(chat);
            let chat_avatar = model.state.avatar_cache.get(chat).and_then(|a| a.as_deref());
            let chat_nickname = model.state.nickname_cache.get(chat).cloned().unwrap_or_else(|| chat.clone());
            let last_msg = model.state.messages.iter()
                .filter(|m| {
                    let from_local = m.from.split('@').next().unwrap_or(&m.from);
                    let to_local = m.to.split('@').next().unwrap_or(&m.to);
                    let my_local = model.state.username.as_deref().unwrap_or("").split('@').next().unwrap_or("");
                    (from_local == my_local && m.to == *chat) || (to_local == my_local && m.from == *chat)
                })
                .last()
                .map(|m| {
                    chrono::DateTime::from_timestamp(m.timestamp as i64, 0)
                        .map(|dt| dt.format("%H:%M").to_string())
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            chats_list = chats_list.push(
                button(row![
                    avatar_circle(chat_avatar, chat, is_selected),
                    Space::with_width(12),
                    column![
                        text(chat_nickname).size(14).shaping(iced::widget::text::Shaping::Advanced),
                        text(chat.clone()).size(11).style(styles::muted_text()).shaping(iced::widget::text::Shaping::Advanced)
                    ],
                    Space::with_width(Length::Fill),
                    text(last_msg).size(11).style(styles::muted_text())
                ].align_y(Alignment::Center))
                 .width(Length::Fill)
                .padding([10, 12])
                .style(move |_, _| button::Style {
                    background: Some((if is_selected { colors::SURFACE } else { iced::Color::TRANSPARENT }).into()),
                    text_color: colors::TEXT_LIGHT,
                    border: Border { radius: 10.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                    ..Default::default()
                })
                .on_press(UiMessage::ChatSelected(chat.clone()))
            );
        }
    }

    container(col.push(Scrollable::new(chats_list).height(Length::Fill)))
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .style(styles::sidebar_container())
        .into()
}
