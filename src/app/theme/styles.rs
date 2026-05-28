use iced::widget::{button, container, text};
use iced::Border;
use super::colors;

pub fn bg_container() -> impl Fn(&iced::Theme) -> container::Style {
    |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        container::Style {
            background: Some((if is_light { 
                iced::Color::from_rgb(0.97, 0.98, 0.99) 
            } else { 
                colors::BG_MAIN 
            }).into()),
            text_color: Some(if is_light { 
                iced::Color::from_rgb(0.1, 0.12, 0.14) 
            } else { 
                colors::TEXT_LIGHT 
            }),
            ..Default::default()
        }
    }
}

pub fn sidebar_container() -> impl Fn(&iced::Theme) -> container::Style {
    |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        container::Style {
            background: Some((if is_light { 
                iced::Color::from_rgb(0.92, 0.94, 0.96) 
            } else { 
                colors::SIDEBAR 
            }).into()),
            ..Default::default()
        }
    }
}

pub fn accent_button() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |theme, _| {
        let is_light = matches!(theme, iced::Theme::Light);
        button::Style {
            background: Some((if is_light { 
                iced::Color::from_rgb(0.2, 0.4, 0.8) 
            } else { 
                colors::ACCENT 
            }).into()),
            text_color: iced::Color::WHITE,
            border: Border { radius: 8.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        }
    }
}

pub fn surface_button() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |theme, _| {
        let is_light = matches!(theme, iced::Theme::Light);
        button::Style {
            background: Some((if is_light { 
                iced::Color::from_rgb(0.85, 0.88, 0.92) 
            } else { 
                colors::SURFACE 
            }).into()),
            text_color: if is_light { 
                iced::Color::from_rgb(0.1, 0.12, 0.14) 
            } else { 
                colors::TEXT_LIGHT 
            },
            border: Border { radius: 8.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        }
    }
}

pub fn muted_text() -> impl Fn(&iced::Theme) -> text::Style {
    |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        text::Style { 
            color: Some(if is_light { 
                iced::Color::from_rgb(0.4, 0.45, 0.5) 
            } else { 
                colors::TEXT_MUTED 
            }) 
        }
    }
}

pub fn msg_bubble(is_mine: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        container::Style {
            background: Some((if is_mine {
                if is_light { iced::Color::from_rgb(0.8, 0.85, 0.95) } else { colors::MSG_MY }
            } else {
                if is_light { iced::Color::from_rgb(0.9, 0.9, 0.92) } else { colors::MSG_THEIR }
            }).into()),
            border: Border { radius: 12.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        }
    }
}