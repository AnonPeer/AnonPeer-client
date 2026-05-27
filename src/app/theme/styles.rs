use iced::widget::{button, container, text};
use iced::Border;
use super::colors; 

pub fn bg_container() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style { 
        background: Some(colors::BG_MAIN.into()), 
        text_color: Some(colors::TEXT_LIGHT), 
        ..Default::default() 
    }
}

pub fn sidebar_container() -> impl Fn(&iced::Theme) -> container::Style {
    |_| container::Style { 
        background: Some(colors::SIDEBAR.into()), 
        ..Default::default() 
    }
}

pub fn accent_button() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, _| button::Style { 
        background: Some(colors::ACCENT.into()), 
        text_color: colors::TEXT_LIGHT, 
        border: Border { radius: 8.0.into(), width: 0.0, color: iced::Color::TRANSPARENT }, 
        ..Default::default() 
    }
}

pub fn surface_button() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_, _| button::Style { 
        background: Some(colors::SURFACE.into()), 
        text_color: colors::TEXT_LIGHT, 
        border: Border { radius: 8.0.into(), width: 0.0, color: iced::Color::TRANSPARENT }, 
        ..Default::default() 
    }
}

pub fn muted_text() -> impl Fn(&iced::Theme) -> text::Style {
    |_| text::Style { color: Some(colors::TEXT_MUTED) }
}

pub fn msg_bubble(is_mine: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style { 
        background: Some((if is_mine { colors::MSG_MY } else { colors::MSG_THEIR }).into()), 
        border: Border { radius: 12.0.into(), width: 0.0, color: iced::Color::TRANSPARENT }, 
        ..Default::default() 
    }
}
