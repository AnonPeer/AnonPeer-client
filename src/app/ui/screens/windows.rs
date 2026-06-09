use iced::widget::{button, column, container, row, text, text_input, Column, Space, horizontal_rule, Image, Scrollable};
use iced::{Alignment, Element, Length};
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::app::theme::{colors, styles};
use crate::app::component::sidebar;
use crate::state::Screen;
use crate::EMOJI_FONT;

pub fn view_screen<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let base_layout = match &model.state.screen {
        Screen::MainMenu { .. } => view_welcome(model),
        Screen::AuthForm { is_register, username, password, .. } => {
            view_auth(model, *is_register, username.as_str(), password.as_str())
        }
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

fn view_hamburger_panel<'a>(_model: &'a Model) -> Element<'a, UiMessage> {
    container(
        column![
            text("Меню").size(18).style(styles::muted_text()),
            horizontal_rule(1),
            Space::with_height(12),
            Space::with_height(6),
            button(
                row![
                    text("🌓").font(EMOJI_FONT).size(16).shaping(iced::widget::text::Shaping::Advanced),
                    text("Сменить тему").size(14)
                ].spacing(8)
            )
            .width(Length::Fill).padding([10, 12]).style(styles::surface_button())
            .on_press(UiMessage::ToggleTheme),
            Space::with_height(Length::Fill),
            button(
                row![
                    text("⚙️").font(EMOJI_FONT).size(16).shaping(iced::widget::text::Shaping::Advanced),
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
                        text("🔔").font(EMOJI_FONT).size(16).shaping(iced::widget::text::Shaping::Advanced),
                        text("Уведомления").size(14)
                    ].spacing(8)
                )
                .width(Length::Fill)
                .padding(12)
                .style(styles::surface_button()),
                button(
                    row![
                        text("🔐").font(EMOJI_FONT).size(16).shaping(iced::widget::text::Shaping::Advanced),
                        text("Конфиденциальность").size(14)
                    ].spacing(8)
                )
                .width(Length::Fill)
                .padding(12)
                .style(styles::surface_button()),
                button(
                    row![
                        text("💾").font(EMOJI_FONT).size(16).shaping(iced::widget::text::Shaping::Advanced),
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
            text("⚙️").font(EMOJI_FONT).size(14).shaping(iced::widget::text::Shaping::Advanced),
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

fn view_auth<'a>(model: &'a Model, is_reg: bool, user: &'a str, pass: &'a str) -> Element<'a, UiMessage> {
    let title = if is_reg { "Регистрация нового профиля" } else { "Авторизация" };
    
    let status_color = if model.state.status.starts_with("Ошибка") || model.state.status.starts_with("Заполните") {
        iced::Color::from_rgb(0.9, 0.3, 0.3) 
    } else if model.state.status.is_empty() {
        iced::Color::TRANSPARENT
    } else {
        iced::Color::from_rgb(0.3, 0.8, 0.3) 
    };

    container(column![
        text(title).size(20),
        Space::with_height(16),
        text("Имя пользователя").size(12).style(styles::muted_text()),
        text_input("Введите логин", user)
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

    column![
        row![
            column![
                text(target).size(18).shaping(iced::widget::text::Shaping::Advanced).font(EMOJI_FONT),
                text(sas_for_text).size(11).style(move |_| iced::widget::text::Style { 
                    color: Some(if sas_for_style.contains("⏳") { colors::TEXT_MUTED } else { colors::SUCCESS }) 
                })
            ]
        ].padding(iced::Padding::default().bottom(10)),
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
