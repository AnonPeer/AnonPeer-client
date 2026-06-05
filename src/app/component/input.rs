use iced::widget::{button, row, text, text_input};
use iced::{Alignment, Element};
use crate::app::message::UiMessage;
use crate::app::theme::styles;

pub fn chat_input<'a>(
    input: &'a str,
    placeholder: &'a str,
    on_input: impl Fn(String) -> UiMessage + 'a,
    on_submit: UiMessage,
    on_attach: UiMessage, // <-- Новый параметр
) -> Element<'a, UiMessage> {
    row![
        button(text("📎").size(16))
            .padding([12, 16])
            .style(styles::surface_button())
            .on_press(on_attach),
        text_input(placeholder, input)
            .on_input(on_input)
            .on_submit(on_submit.clone())
            .padding(12)
            .size(14),
        button(text("Отправить").size(14))
            .padding([12, 20])
            .style(styles::accent_button())
            .on_press(on_submit)
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}