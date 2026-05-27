use iced::Element;
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::app::theme::styles;
use super::screens::windows;
pub fn view<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    iced::widget::container(windows::view_screen(model)).width(iced::Length::Fill).height(iced::Length::Fill).style(styles::bg_container()).into()
}