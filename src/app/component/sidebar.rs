use iced::widget::{button, column, container, row, text, text_input, Column, Scrollable, Space};
use iced::{Alignment, Border, Element, Length};
use crate::app::message::UiMessage;
use crate::app::model::Model;
use crate::app::theme::{colors, styles};

pub fn view<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let my_name = model.state.username.as_deref().unwrap_or("Аноним");
    let chats = &model.state.chats;
    let selected = model.selected_chat.as_ref();

    let mut col = Column::new().spacing(10).padding(12).height(Length::Fill);

    col = col.push(row![
        column![text(my_name).size(15), text("online").size(11).style(styles::muted_text())],
        Space::with_width(Length::Fill)
    ].align_y(Alignment::Center));

    let search_input: Element<'a, UiMessage> = text_input("🔍 Искать пользователя...", &model.search_query)
        .on_input(UiMessage::SearchInputChanged)
        .padding(8).size(13).width(Length::Fill).into();

    col = col.push(search_input);

    if !model.search_matches.is_empty() {
        let mut dropdown = Column::new().spacing(2).padding([4, 0]);
        for name in &model.search_matches {
            let name_owned = name.clone(); 
            let name_for_text = name.clone(); 
            
            dropdown = dropdown.push(
                button(text(name_for_text).size(13))
                    .width(Length::Fill)
                    .padding(8)
                    .style(|_, _| button::Style {
                        background: Some(iced::Color::TRANSPARENT.into()),
                        ..Default::default()
                    })
                    .on_press(UiMessage::SearchResultSelected(name_owned))
            );
        }
        col = col.push(
            container(dropdown)
                .width(Length::Fill)
                .padding(4)
                .style(|_| container::Style {
                    background: Some(colors::SURFACE.into()),
                    border: Border { radius: 6.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                    ..Default::default()
                })
        );
    }

    col = col.push(Space::with_height(6));
    col = col.push(
        button(text("+ Создать чат").size(14))
            .width(Length::Fill).padding(10)
            .style(styles::accent_button())
            .on_press(UiMessage::OpenNewChatScreen)
    );
    col = col.push(Space::with_height(5));

    let mut chats_list = Column::new().spacing(4);
    if chats.is_empty() {
        chats_list = chats_list.push(
            container(text("Нет активных чатов").size(13).style(styles::muted_text()))
                .padding(10).center_x(Length::Fill)
        );
    } else {
        for chat in chats {
            let is_selected = selected == Some(chat);
            let first = chat.chars().next().unwrap_or('?').to_uppercase().to_string();
            chats_list = chats_list.push(
                button(row![
                    container(text(first).size(14))
                        .center_x(32).center_y(32)
                        .style(|_| container::Style {
                            background: Some(colors::SURFACE.into()),
                            border: iced::Border { radius: 16.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                            ..Default::default()
                        }),
                    Space::with_width(8),
                    text(chat).size(14)
                ].align_y(Alignment::Center))
                .width(Length::Fill).padding(8)
                .style(move |_, _| button::Style {
                    background: Some((if is_selected { colors::ACCENT } else { iced::Color::TRANSPARENT }).into()),
                    text_color: colors::TEXT_LIGHT,
                    border: iced::Border { radius: 8.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                    ..Default::default()
                })
                .on_press(UiMessage::ChatSelected(chat.clone()))
            );
        }
    }

    container(col.push(Scrollable::new(chats_list).height(Length::Fill)))
        .width(Length::Fixed(260.0))
        .height(Length::Fill)
        .style(styles::sidebar_container())
        .into()
}