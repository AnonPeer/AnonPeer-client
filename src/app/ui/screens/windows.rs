use iced::widget::{button, column, container, row, text, text_input, Column, Space, horizontal_rule, Image, Scrollable};
use iced::{Alignment, Element, Length};
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::app::theme::{colors, styles};
use crate::app::component::sidebar;
use crate::domain::state::Screen;
use crate::NOTO_SANS;
use base64::Engine as _;

pub fn view_screen<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    

        
    let base_layout = match &model.state.screen {
        Screen::MainMenu { .. } => view_welcome(model),
        Screen::AuthForm { is_register, nickname, username, password, .. } => {
            view_auth(model, *is_register, nickname.as_str(), username.as_str(), password.as_str())
        }
        Screen::UserProfile { username } => view_profile(model, username.as_str()),
        Screen::EditProfile => view_edit_profile(model),
        _ => view_app_shell(model),
    };

    let mut final_view = base_layout;





    if model.show_settings {
        let dimmer = button(row![])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_, _| iced::widget::button::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4).into()),
                ..Default::default()
            })
            .on_press(UiMessage::ToggleSettings);

        let bottom_panel = container(view_settings_content(model))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::End);

        final_view = iced::widget::stack![final_view, dimmer, bottom_panel].into();
    }

    if let Some(img_id) = model.expanded_image_id {
        let handle_opt = {
            let cache = model.image_cache.read().unwrap();
            cache.get(&img_id).cloned()
        };







        if let Some(handle) = handle_opt {
            let img_dimmer = button(row![])
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_, _| iced::widget::button::Style {
                    background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.85).into()),
                    ..Default::default()
                })
                .on_press(UiMessage::CloseExpandedImage);

            let expanded_img = iced::widget::Image::new(handle)
                .width(Length::Fill)
                .height(Length::Fill);

            let img_content = container(expanded_img)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill);

            final_view = iced::widget::stack![final_view, img_dimmer, img_content].into();
        }
    }

    final_view
}



fn view_app_shell<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let sidebar = sidebar::view(model);

    let hamburger_icon = column![
        container(horizontal_rule(2)).width(18).height(2),
        container(horizontal_rule(2)).width(18).height(2),
        container(horizontal_rule(2)).width(18).height(2),
    ]
    .spacing(4)
    .align_x(Alignment::Center);

    let hamburger_button = button(hamburger_icon)
        .padding(10)
        .style(styles::surface_button())
        .on_press(UiMessage::ToggleHamburgerMenu);

    let burger_panel = column![hamburger_button]
        .width(Length::Fixed(60.0))
        .align_x(Alignment::Center);

    let drawer_width = if model.hamburger_open { 240.0 } else { 0.0 };
    let drawer = container(if model.hamburger_open {
        view_hamburger_panel(model)
    } else {
        column![].into()
    })
    .width(Length::Fixed(drawer_width))
    .height(Length::Fill);

    let content = match &model.state.screen {
        Screen::ChatView { target, input } => view_chat_view(model, target.as_str(), input.as_str()),
        Screen::NewChat { input } => view_new_chat(model, input.as_str()),
        _ => view_empty(),
    };

    let main_area = column![content]
        .width(Length::Fill)
        .height(Length::Fill);

    row![burger_panel, drawer, sidebar, main_area]
        .height(Length::Fill)
        .into()
}

fn view_hamburger_panel<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let my_username = model.state.username.as_deref().unwrap_or("");
    container(
        column![
            text("Меню").size(18).style(styles::muted_text()),
            horizontal_rule(1),
            Space::with_height(12),
            button(
                row![
                    text("👤").font(NOTO_SANS).size(16).shaping(iced::widget::text::Shaping::Advanced),
                    text("Мой профиль").size(14)
                ].spacing(8)
            )
            .width(Length::Fill).padding([10, 12]).style(styles::surface_button())
            .on_press(UiMessage::ViewProfile(my_username.to_string())),
            Space::with_height(6),
            button(
                row![
                    text("🌓").font(NOTO_SANS).size(16).shaping(iced::widget::text::Shaping::Advanced),
                    text("Сменить тему").size(14)
                ].spacing(8)
            )
            .width(Length::Fill).padding([10, 12]).style(styles::surface_button())
            .on_press(UiMessage::ToggleTheme),
            Space::with_height(Length::Fill),
            button(
                row![
                    text("⚙️").font(NOTO_SANS).size(16).shaping(iced::widget::text::Shaping::Advanced),
                    text("Настройки").size(14)
                ].spacing(8)
            )
            .width(Length::Fill).padding([10, 12]).style(styles::surface_button())
            .on_press(UiMessage::ToggleSettings),
        ].spacing(8).padding(16)
    )
    .width(Length::Fixed(240.0))
    .height(Length::Fill)
    .style(styles::sidebar_container())
    .into()
}

fn view_settings_content<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let content = container(
        column![
            container(row![
                text("Настройки").size(20).style(styles::muted_text()),
                Space::with_width(Length::Fill),
                button(text("✕").size(16))
                    .padding(8)
                    .style(styles::surface_button())
                    .on_press(UiMessage::ToggleSettings)
            ])
            .width(Length::Fill)
            .padding([16, 20]),
            horizontal_rule(1),
            column![
                text("Адрес WS сервера").size(12).style(styles::muted_text()),
                text_input("ws://ip:port/ws", &model.state.server_address)
                    .on_input(UiMessage::ServerAddressChanged)
                    .padding(10),
                Space::with_height(10),
                button(
                    row![
                        text("🔔").font(NOTO_SANS).size(16).shaping(iced::widget::text::Shaping::Advanced),
                        text("Уведомления").size(14)
                    ].spacing(8)
                )
                .width(Length::Fill)
                .padding(12)
                .style(styles::surface_button()),
                button(
                    row![
                        text("🔐").font(NOTO_SANS).size(16).shaping(iced::widget::text::Shaping::Advanced),
                        text("Конфиденциальность").size(14)
                    ].spacing(8)
                )
                .width(Length::Fill)
                .padding(12)
                .style(styles::surface_button()),
                button(
                    row![
                        text("💾").font(NOTO_SANS).size(16).shaping(iced::widget::text::Shaping::Advanced),
                        text("Данные и память").size(14)
                    ].spacing(8)
                )
                .width(Length::Fill)
                .padding(12)
                .style(styles::surface_button()),
                button(text("Выйти").size(14))
                    .width(Length::Fill)
                    .padding(12)
                    .style(styles::surface_button())
                    .on_press(UiMessage::Logout)
            ]
            .spacing(8)
            .padding([12, 20])
        ]
        .spacing(0)
        .width(Length::Fill)
    )
    .width(Length::Fill)
    .max_width(600.0)
    .height(Length::Fill)
    .max_height(500.0)
    .style(styles::bg_container());

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn view_welcome<'a>(_model: &'a Model) -> Element<'a, UiMessage> {
    const ICON_BYTES: &[u8] = include_bytes!("../../../ico.ico");
    let logo = Image::new(iced::widget::image::Handle::from_bytes(ICON_BYTES)).width(Length::Fixed(128.0)).height(Length::Fixed(128.0));
    let main_content = column![
        logo,
        Space::with_height(10),
        text("AnonPeer").size(32),
        text("Децентрализованный анонимный мессенджер").size(13).style(styles::muted_text()),
        Space::with_height(30),
        button(text("Вход в систему").size(14)).width(Length::Fill).padding(12).style(styles::accent_button()).on_press(UiMessage::MainMenuSelect(0)),
        Space::with_height(10),
        button(text("Создать новый аккаунт").size(14)).width(Length::Fill).padding(12).style(styles::surface_button()).on_press(UiMessage::MainMenuSelect(1))
    ]
    .align_x(Alignment::Center)
    .max_width(320);

    let settings_button = button(
        row![
            text("⚙️").font(NOTO_SANS).size(14).shaping(iced::widget::text::Shaping::Advanced),
            text("Настройки сервера").size(13)
        ].spacing(6)
    )
    .padding([8, 14])
    .style(styles::surface_button())
    .on_press(UiMessage::ToggleSettings);

    let footer_row = row![
        Space::with_width(Length::Fill),
        settings_button
    ]
    .padding(20);

    column![
        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        footer_row
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn view_auth<'a>(model: &'a Model, is_reg: bool, nickname: &'a str, user: &'a str, pass: &'a str) -> Element<'a, UiMessage> {
    let title = if is_reg { "Регистрация нового профиля" } else { "Авторизация" };
    let status_color = if model.state.status.starts_with("Ошибка") || model.state.status.starts_with("Заполните") {
        iced::Color::from_rgb(0.9, 0.3, 0.3) 
    } else if model.state.status.is_empty() {
        iced::Color::TRANSPARENT
    } else {
        iced::Color::from_rgb(0.3, 0.8, 0.3) 
    };

    let nickname_field = if is_reg {
        column![
            text("Отображаемое имя (может повторяться)").size(12).style(styles::muted_text()),
            text_input("Введите никнейм", nickname)
                .on_input(UiMessage::AuthNicknameChanged)
                .padding(10),
            Space::with_height(12),
        ]
    } else {
        column![]
    };

    container(column![
        text(title).size(20),
        Space::with_height(16),
        nickname_field, 
        text("Уникальный логин (не повторяется)").size(12).style(styles::muted_text()),
        text_input("Введите логин (username)", user)
            .on_input(UiMessage::AuthUsernameChanged)
            .padding(10),
        Space::with_height(12),
        text("Пароль").size(12).style(styles::muted_text()),
        row![
            text_input("Введите пароль", pass)
                .on_input(UiMessage::AuthPasswordChanged)
                .secure(!model.password_visible)
                .padding(10),
            button(text(if model.password_visible { "Скрыть" } else { "Показать" }).size(12))
                .padding(10)
                .on_press(UiMessage::AuthTogglePasswordVisibility)
        ].spacing(6),
        Space::with_height(10),
        text(&model.state.status).size(12).style(move |_| iced::widget::text::Style { 
            color: Some(status_color) 
        }),
        Space::with_height(10),
        row![
            button(text("Назад").size(14)).padding([10,20]).on_press(UiMessage::AuthBack),
            Space::with_width(Length::Fill),
            button(text("Продолжить").size(14)).padding([10,24]).style(styles::accent_button()).on_press(UiMessage::AuthSubmit)
        ]
    ].max_width(360).spacing(4))
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn view_chat_view<'a>(model: &'a Model, target: &'a str, input: &'a str) -> Element<'a, UiMessage> {
    let my = model.state.username.as_deref().unwrap_or("");
    let my_local = my.split('@').next().unwrap_or(my); 
    
    let filtered: Vec<_> = model.state.messages.iter()
        .filter(|m| {
            if m.ciphertext.is_empty() { return false; }
            
            let from_local = m.from.split('@').next().unwrap_or(&m.from);
            let to_local = m.to.split('@').next().unwrap_or(&m.to);
            
            (from_local == my_local && m.to == target) || (to_local == my_local && m.from == target)
        })
        .collect();
    
    let mut msg_list = Column::new().spacing(10).height(Length::Shrink);

    let sas_display = if let Some(sas) = model.state.get_sas_code(target) {
        format!("🔒 Код безопасности: {} (совпадает = безопасно)", sas)
    } else {
        "⏳ Обмен ключами...".to_string()
    };

    let sas_for_text = sas_display.clone();
    let sas_for_style = sas_display;

    if filtered.is_empty() {
        msg_list = msg_list.push(
            container(text("История сообщений пуста").size(13).style(styles::muted_text()))
                .center_x(Length::Fill)
                .padding(40)
        );
    } else {
        for m in filtered {
            msg_list = msg_list.push(crate::app::component::message::view_bubble(m, model, my));
        }
    }

    let target_nickname = model.state.nickname_cache.get(target).cloned().unwrap_or_else(|| target.to_string());
    let target_avatar = model.state.avatar_cache.get(target).and_then(|a| a.as_deref());

    let target_first = target_nickname.chars().next().unwrap_or('?').to_uppercase().to_string();
    let header_avatar_size = 32.0;
    let header_avatar: Element<'_, UiMessage> = if let Some(b64) = target_avatar {
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(b64) {
            let handle = iced::widget::image::Handle::from_bytes(bytes);
            container(Image::new(handle).width(Length::Fixed(header_avatar_size)).height(Length::Fixed(header_avatar_size)))
                .width(Length::Fixed(header_avatar_size))
                .height(Length::Fixed(header_avatar_size))
                .style(move |_: &iced::Theme| container::Style {
                    border: iced::Border { radius: (header_avatar_size / 2.0).into(), width: 0.0, color: iced::Color::TRANSPARENT },
                    ..Default::default()
                })
                .into()
        } else {
            make_letter_avatar(target_first.clone(), header_avatar_size).into()
        }
    } else {
        make_letter_avatar(target_first.clone(), header_avatar_size).into()
    };

    let header = button(
        row![
            header_avatar,
            Space::with_width(10),
            column![
                text(target_nickname).size(16).shaping(iced::widget::text::Shaping::Advanced),
                text(sas_for_text).size(11).style(move |_| iced::widget::text::Style { 
                    color: Some(if sas_for_style.contains("⏳") { colors::TEXT_MUTED } else { colors::SUCCESS }) 
                })
            ]
        ].align_y(Alignment::Center)
    )
    .padding(iced::Padding::default().bottom(10).top(4))
    .on_press(UiMessage::ViewProfile(target.to_string()));

    column![
        header,
        horizontal_rule(1),
        Space::with_height(10),
        Scrollable::new(msg_list)
            .id(model.scroll_id.clone())
            .height(Length::Fill),
        Space::with_height(10),
        crate::app::component::input::chat_input(
            input,
            "Напишите сообщение...",
            |v| UiMessage::ChatViewInputChanged(v),
            UiMessage::ChatViewSend,
            UiMessage::PickImage,
        )
    ]
    .height(Length::Fill)
    .into()
}

fn view_new_chat<'a>(_model: &'a Model, input: &'a str) -> Element<'a, UiMessage> {
    container(column![
        text("Создать секретный диалог").size(18),
        Space::with_height(14),
        text("Введите логин или логин@сервер").size(12).style(styles::muted_text()),
        text_input("user или user@domain.com", input)
            .on_input(UiMessage::NewChatInputChanged)
            .on_submit(UiMessage::NewChatSubmit)
            .padding(12),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn view_search_bar<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let search_input: Element<'a, UiMessage> = text_input("🔍 Искать по нику...", &model.state.server_address) 
        .on_input(|v| UiMessage::ServerAddressChanged(v)) 
        .on_submit(UiMessage::SaveServerAddress)
        .padding(8)
        .size(13)
        .width(Length::Fixed(220.0))
        .into();

    let search_btn: Element<'a, UiMessage> = button(text("Найти").size(12))
        .padding([6, 14])
        .style(styles::accent_button())
        .on_press(UiMessage::SaveServerAddress)
        .into();

    row![search_input, search_btn]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
}

fn view_empty<'a>() -> Element<'a, UiMessage> {
    container(text("Выберите чат или создайте новый для начала общения").size(14).style(styles::muted_text()))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}


fn view_profile<'a>(model: &'a Model, username: &'a str) -> Element<'a, UiMessage> {
    let nickname = model.state.nickname_cache.get(username).cloned().unwrap_or_else(|| "Загрузка...".to_string());
    let bio = model.state.bio_cache.get(username).cloned().unwrap_or_default();
    let avatar = model.state.avatar_cache.get(username).and_then(|a| a.as_deref());
    let last_seen = model.state.last_seen_cache.get(username).and_then(|v| *v);
    let is_me = model.state.username.as_deref() == Some(username);

    let avatar_widget = render_avatar_large(avatar, &nickname);

    let last_seen_text = if is_me {
        "В сети".to_string()
    } else if let Some(ts) = last_seen {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let diff = now.saturating_sub(ts);
        if diff < 60 {
            "в сети".to_string()
        } else if diff < 3600 {
            format!("был(а) в сети {} мин. назад", diff / 60)
        } else if diff < 86400 {
            format!("был(а) в сети {} ч. назад", diff / 3600)
        } else {
            format!("был(а) в сети {} дн. назад", diff / 86400)
        }
    } else {
        String::new()
    };

    let mut profile_col = column![
        row![
            button(text("← Назад").size(14))
                .padding([10, 20])
                .style(styles::surface_button())
                .on_press(UiMessage::CloseProfile),
            Space::with_width(Length::Fill),
            if is_me {
                button(text("✏️ Редактировать").size(13).shaping(iced::widget::text::Shaping::Advanced))
                    .padding([8, 14])
                    .style(styles::accent_button())
                    .on_press(UiMessage::OpenEditProfile)
            } else {
                button(text("")).padding(0)
            }
        ],
        Space::with_height(30),
        avatar_widget,
        Space::with_height(16),
        text(nickname).size(26).shaping(iced::widget::text::Shaping::Advanced),
        text(format!("@{}", username)).size(15).style(styles::muted_text()),
        Space::with_height(8),
        text(last_seen_text).size(13).style(styles::muted_text()),
    ].align_x(Alignment::Center);

    if !bio.is_empty() {
        profile_col = profile_col
            .push(Space::with_height(20))
            .push(
                container(column![
                    text("О себе").size(12).style(styles::muted_text()),
                    Space::with_height(4),
                    text(bio).size(15).shaping(iced::widget::text::Shaping::Advanced),
                ].spacing(4))
                .padding(16)
                .width(Length::Fill)
                .max_width(400.0)
                .style(styles::surface_container())
            );
    }

    if !is_me {
        profile_col = profile_col
            .push(Space::with_height(24))
            .push(
                button(text("Написать сообщение").size(14))
                    .padding([12, 32])
                    .style(styles::accent_button())
                    .on_press(UiMessage::ChatSelected(username.to_string()))
            );
    }

    container(Scrollable::new(profile_col).spacing(4))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn view_edit_profile<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let current_avatar = model.state.edit_avatar_base64.as_deref();
    let nickname = model.state.nickname.as_deref().unwrap_or("Аноним");

    let avatar_section: Element<'a, UiMessage> = if let Some(b64) = current_avatar {
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(b64) {
            let handle = iced::widget::image::Handle::from_bytes(bytes);
            container(
                Image::new(handle)
                    .width(Length::Fixed(100.0))
                    .height(Length::Fixed(100.0))
            )
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0))
            .style(|_: &iced::Theme| container::Style {
                border: iced::Border { radius: 50.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                ..Default::default()
            })
            .into()
        } else {
            render_avatar_large(None, nickname)
        }
    } else {
        render_avatar_large(None, nickname)
    };

    let content = column![
        row![
            button(text("← Назад").size(14))
                .padding([10, 20])
                .style(styles::surface_button())
                .on_press(UiMessage::CloseProfile),
            Space::with_width(Length::Fill),
            text("Редактирование профиля").size(18).style(styles::muted_text()),
            Space::with_width(Length::Fill),
        ],
        Space::with_height(30),
        container(avatar_section)
            .center_x(Length::Fill),
        Space::with_height(10),
        button(text("📷 Изменить аватар").size(13))
            .padding([8, 16])
            .style(styles::surface_button())
            .on_press(UiMessage::PickImage),
        Space::with_height(24),
        text("Имя").size(13).style(styles::muted_text()),
        text(nickname).size(16).shaping(iced::widget::text::Shaping::Advanced),
        Space::with_height(4),
        text("(имя задаётся при регистрации)").size(11).style(styles::muted_text()),
        Space::with_height(20),
        text("О себе").size(13).style(styles::muted_text()),
        container(
            text_input("Расскажите о себе...", &model.state.edit_bio)
                .on_input(UiMessage::EditProfileBioChanged)
                .padding(12)
                .width(Length::Fill)
        ).width(Length::Fill).max_width(400.0),
        Space::with_height(30),
        button(text("Сохранить").size(14))
            .padding([12, 32])
            .style(styles::accent_button())
            .on_press(UiMessage::EditProfileSave),
    ].spacing(4).max_width(420);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn make_letter_avatar(letter: String, size: f32) -> iced::widget::Container<'static, UiMessage> {
    container(text(letter).size(14).shaping(iced::widget::text::Shaping::Advanced))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .center_x(Length::Fixed(size))
        .center_y(Length::Fixed(size))
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(colors::ACCENT)),
            text_color: Some(colors::TEXT_LIGHT),
            border: iced::Border { radius: (size / 2.0).into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        })
}

fn render_avatar_large<'a>(avatar_b64: Option<&str>, label: &str) -> Element<'a, UiMessage> {
    let size = 100.0;
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
                border: iced::Border { radius: (size / 2.0).into(), width: 0.0, color: iced::Color::TRANSPARENT },
                ..Default::default()
            })
            .into();
        }
    }
    let first = label.chars().next().unwrap_or('?').to_uppercase().to_string();
    container(text(first).size(40).shaping(iced::widget::text::Shaping::Advanced))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .center_x(Length::Fixed(size))
        .center_y(Length::Fixed(size))
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(colors::ACCENT)),
            text_color: Some(colors::TEXT_LIGHT),
            border: iced::Border { radius: (size / 2.0).into(), width: 0.0, color: iced::Color::TRANSPARENT },
            ..Default::default()
        })
        .into()
}