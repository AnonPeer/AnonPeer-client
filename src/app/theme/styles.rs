use iced::widget::{button, container, text, text_input};
use iced::{Border, Shadow, Vector};
use super::colors;

pub fn bg_container() -> impl Fn(&iced::Theme) -> container::Style {
    |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        container::Style {
            background: Some((if is_light { iced::Color::from_rgb(0.97, 0.98, 0.99) } else { colors::BG_MAIN }).into()),
            text_color: Some(if is_light { iced::Color::from_rgb(0.1, 0.12, 0.14) } else { colors::TEXT_LIGHT }),
            ..Default::default()
        }
    }
}

pub fn sidebar_container() -> impl Fn(&iced::Theme) -> container::Style {
    |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        container::Style {
            background: Some((if is_light { iced::Color::from_rgb(0.92, 0.94, 0.96) } else { colors::SIDEBAR }).into()),
            ..Default::default()
        }
    }
}

pub fn surface_container() -> impl Fn(&iced::Theme) -> container::Style {
    |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        container::Style {
            background: Some((if is_light { iced::Color::from_rgb(0.95, 0.96, 0.98) } else { colors::SURFACE }).into()),
            border: Border { radius: 12.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            shadow: Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }
    }
}

pub fn accent_button() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |theme, status| {
        let is_light = matches!(theme, iced::Theme::Light);
        let bg = if is_light { iced::Color::from_rgb(0.2, 0.4, 0.8) } else { colors::ACCENT };
        let bg_hover = if is_light { iced::Color::from_rgb(0.15, 0.3, 0.7) } else { colors::ACCENT_HOVER };
        
        button::Style {
            background: Some((if matches!(status, button::Status::Hovered) { bg_hover } else { bg }).into()),
            text_color: iced::Color::WHITE,
            border: Border { radius: 10.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        }
    }
}

pub fn surface_button() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |theme, status| {
        let is_light = matches!(theme, iced::Theme::Light);
        let bg = if is_light { iced::Color::from_rgb(0.85, 0.88, 0.92) } else { colors::SURFACE };
        let bg_hover = if is_light { iced::Color::from_rgb(0.8, 0.83, 0.87) } else { iced::Color::from_rgb(0.20, 0.21, 0.26) };

        button::Style {
            background: Some((if matches!(status, button::Status::Hovered) { bg_hover } else { bg }).into()),
            text_color: if is_light { iced::Color::from_rgb(0.1, 0.12, 0.14) } else { colors::TEXT_LIGHT },
            border: Border { radius: 10.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        }
    }
}

pub fn icon_button() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |theme, status| {
        let is_light = matches!(theme, iced::Theme::Light);
        let bg_hover = if is_light { iced::Color::from_rgba(0.0, 0.0, 0.0, 0.05) } else { iced::Color::from_rgba(1.0, 1.0, 1.0, 0.05) };
        button::Style {
            background: Some((if matches!(status, button::Status::Hovered) { bg_hover } else { iced::Color::TRANSPARENT }).into()),
            text_color: if is_light { iced::Color::from_rgb(0.1, 0.12, 0.14) } else { colors::TEXT_LIGHT },
            border: Border { radius: 8.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        }
    }
}

pub fn modern_text_input() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    move |theme, status| {
        let is_light = matches!(theme, iced::Theme::Light);
        
        let bg = if is_light { 
            iced::Color::from_rgb(0.95, 0.96, 0.98) 
        } else { 
            colors::SURFACE 
        };
        
        let border_color = if matches!(status, text_input::Status::Focused) { 
            colors::ACCENT 
        } else { 
            iced::Color::TRANSPARENT 
        };

        let text_color = if is_light { 
            iced::Color::from_rgb(0.1, 0.12, 0.14) 
        } else { 
            colors::TEXT_LIGHT 
        };

        text_input::Style {
            background: iced::Background::Color(bg),
            border: iced::Border { 
                radius: 10.0.into(), 
                width: 1.5, 
                color: border_color 
            },
            icon: colors::TEXT_MUTED,
            placeholder: colors::TEXT_MUTED,
            value: text_color,
            selection: colors::ACCENT,
        }
    }
}

pub fn muted_text() -> impl Fn(&iced::Theme) -> text::Style {
    |theme| {
        let is_light = matches!(theme, iced::Theme::Light);
        text::Style {
            color: Some(if is_light { iced::Color::from_rgb(0.4, 0.45, 0.5) } else { colors::TEXT_MUTED })
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
            border: Border { 
                radius: 16.0.into(), 
                width: 0.0, 
                color: iced::Color::TRANSPARENT 
            },
            shadow: Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            ..Default::default()
        }
    }
}