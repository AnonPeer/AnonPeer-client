use iced::widget::{button, column, container, row, text, text_input, Column, Scrollable, Space};
use iced::{Alignment, Border, Element, Length};
use crate::app::message::UiMessage;
use crate::app::model::Model;
use crate::app::theme::{colors, styles};
use crate::EMOJI_FONT;

pub fn view<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let my_name = model.state.username.as_deref().unwrap_or("Аноним");
    let chats = &model.state.chats;
    let selected = model.selected_chat.as_ref();
    
    let mut col = Column::new().spacing(12).padding(16).height(Length::Fill);

    col = col.push(row![
        column![
            text(my_name).size(16).shaping(iced::widget::text::Shaping::Advanced).font(EMOJI_FONT),
            text("в сети").size(12).style(styles::muted_text())
        ],
        Space::with_width(Length::Fill)
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
        for name in &model.search_matches {
            let name_owned = name.clone(); 
            dropdown = dropdown.push(
                button(text(name.clone()).size(14).shaping(iced::widget::text::Shaping::Advanced).font(EMOJI_FONT))
                    .width(Length::Fill)
                    .padding([8, 12])
                    .style(styles::surface_button())
                    .on_press(UiMessage::SearchResultSelected(name_owned))
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
            text("").size(18),
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
            let first = chat.chars().next().unwrap_or('?').to_uppercase().to_string();
            
            chats_list = chats_list.push(
                button(row![
                    container(text(first).size(16).shaping(iced::widget::text::Shaping::Advanced))
                        .center_x(36).center_y(36)
                        .style(move |_| container::Style { 
                            background: Some(iced::Background::Color(if is_selected { colors::ACCENT } else { colors::SURFACE })),
                            text_color: Some(colors::TEXT_LIGHT),
                            border: Border { radius: 18.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                            ..Default::default()
                        }),
                    Space::with_width(12),
                    text(chat).size(14).shaping(iced::widget::text::Shaping::Advanced).font(EMOJI_FONT)
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