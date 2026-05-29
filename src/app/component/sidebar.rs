use iced::widget::{button, column, container, row, text, Column, Scrollable, Space};
use iced::{Alignment, Element, Length};
use crate::app::message::UiMessage;
use crate::app::theme::{colors, styles};

pub fn view<'a>(
    username: Option<&'a String>,
    chats: &'a [String],
    selected: Option<&'a String>,
) -> Element<'a, UiMessage> {
    let my_name: &str = username.map(|s| s.as_str()).unwrap_or("Аноним");
    
    let mut sidebar_col = Column::new().spacing(10).padding(12).height(Length::Fill);
    
    sidebar_col = sidebar_col.push(
        row![
            column![
                text(my_name).size(15),
                text("online").size(11).style(styles::muted_text())
            ],
            Space::with_width(Length::Fill),
        ].align_y(Alignment::Center)
    );

    sidebar_col = sidebar_col.push(
        button(text("+ Создать чат").size(14))
            .width(Length::Fill)
            .padding(10)
            .style(styles::accent_button())
            .on_press(UiMessage::OpenNewChatScreen)
    );

    sidebar_col = sidebar_col.push(Space::with_height(5));

    let mut chats_list = Column::new().spacing(4);

    if chats.is_empty() {
        chats_list = chats_list.push(
            container(text("Нет активных чатов").size(13).style(styles::muted_text()))
                .padding(10)
                .center_x(Length::Fill)
        );
    } else {
        for chat in chats {
            let is_selected = selected == Some(chat);
            let first = chat.chars().next().unwrap_or('?').to_uppercase().to_string();

            chats_list = chats_list.push(
                button(
                    row![
                        container(text(first).size(14))
                            .center_x(32).center_y(32)
                            .style(|_| container::Style {
                                background: Some(colors::SURFACE.into()),
                                border: iced::Border { radius: 16.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                                ..Default::default()
                            }),
                        Space::with_width(8),
                        text(chat).size(14)
                    ].align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .padding(8)
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

    container(
        sidebar_col.push(Scrollable::new(chats_list).height(Length::Fill))
    )
    .width(Length::Fixed(260.0))
    .height(Length::Fill)
    .style(styles::sidebar_container())
    .into()
}