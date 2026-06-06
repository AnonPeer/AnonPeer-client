use iced::widget::{button, row, text, text_input, container};
use iced::{Alignment, Element, Length};
use crate::app::message::UiMessage;
use crate::app::theme::styles;
use crate::EMOJI_FONT;

pub fn chat_input<'a>(
    input: &'a str,
    placeholder: &'a str,
    on_input: impl Fn(String) -> UiMessage + 'a,
    on_submit: UiMessage,
    on_attach: UiMessage,
) -> Element<'a, UiMessage> {
    container(
        row![
            button(text("").size(18).shaping(iced::widget::text::Shaping::Advanced).font(EMOJI_FONT))
                .padding(10)
                .style(styles::icon_button())
                .on_press(on_attach),
            
            text_input(placeholder, input)
                .on_input(on_input)
                .on_submit(on_submit.clone())
                .padding([10, 14])
                .size(15)
                .width(Length::Fill)
                .style(styles::modern_text_input()),
            
            button(text("󰒊").size(18).shaping(iced::widget::text::Shaping::Advanced).font(EMOJI_FONT))
                .padding(10)
                .style(styles::accent_button())
                .on_press(on_submit)
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    )
    .padding([8, 16])
    .width(Length::Fill)
    .into()
}