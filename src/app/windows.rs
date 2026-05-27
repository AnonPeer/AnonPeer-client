use iced::widget::{button, column, container, row, text, text_input, Column, Space, horizontal_rule, Image, Scrollable};
use iced::{Alignment, Element, Length};
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::app::theme::styles;
use crate::app::component::sidebar;
use crate::state::Screen;

pub fn view_screen<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    match &model.state.screen {
        Screen::MainMenu { .. } => view_welcome(model),
        Screen::AuthForm { is_register, username, password, .. } => {
            view_auth(model, *is_register, username.as_str(), password.as_str())\n        }
        _ => view_app_shell(model),
    }
}

fn view_app_shell<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    let sb = sidebar::view(
        model.state.username.as_ref(),
        &model.state.chats,
        model.selected_chat.as_ref(),
    );
    let content = match &model.state.screen {
        Screen::ChatView { target, input } => {
            view_chat_view(model, target.as_str(), input.as_str())
        }
        Screen::NewChat { input } => view_new_chat(model, input.as_str()),
        _ => view_empty(),
    };
    iced::widget::row![sb, content].height(Length::Fill).into()
}

fn view_welcome<'a>(_model: &'a Model) -> Element<'a, UiMessage> {
    container(
        column![
            text("AnonPeer").size(48),
            Space::with_height(8),
            text("Децентрализованный анонимный мессенджер").size(16),
            Space::with_height(32),
            button(text("Вход").size(14))
                .padding([10, 24])
                .style(styles::accent_button())
                .on_press(UiMessage::MainMenuSelect(0)),
            Space::with_height(12),
            button(text("Регистрация").size(14))
                .padding([10, 24])
                .on_press(UiMessage::MainMenuSelect(1)),
        ]
        .alignment(Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn view_auth<'a>(model: &'a Model, is_register: bool, u: &'a str, p: &'a str) -> Element<'a, UiMessage> {
    let title = if is_register { "Регистрация нового аккаунта" } else { "Вход в аккаунт" };
    let submit_label = if is_register { "Зарегистрироваться" } else { "Войти" };
    container(
        column![
            text(title).size(20),
            Space::with_height(20),
            text_input("Имя пользователя", u)
                .on_input(UiMessage::AuthUsernameChanged)
                .on_submit(UiMessage::AuthSubmit)
                .padding(12),
            Space::with_height(12),
            row![
                text_input("Пароль", p)
                    .secure(!model.password_visible)
                    .on_input(UiMessage::AuthPasswordChanged)
                    .on_submit(UiMessage::AuthSubmit)
                    .padding(12)
                    .width(Length::Fill),
                button(text(if model.password_visible { "Скрыть" } else { "Показать" }).size(12))
                    .padding(12)
                    .on_press(UiMessage::AuthTogglePasswordVisibility)
            ].spacing(8),
            Space::with_height(20),
            if !model.state.status.is_empty() {
                column![text(&model.state.status).size(13), Space::with_height(14)]
            } else {
                column![]
            },
            row![
                button(text("Назад").size(13)).padding([10,20]).on_press(UiMessage::AuthBack),
                Space::with_width(Length::Fill),
                button(text(submit_label).size(13))
                    .padding([10,20])
                    .style(styles::accent_button())
                    .on_press(UiMessage::AuthSubmit)
            ]
        ]
        .max_width(360)
        .spacing(4)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn view_chat_view<'a>(model: &'a Model, target: &'a str, input: &'a str) -> Element<'a, UiMessage> {
    let mut msg_list = column![].spacing(10).padding(10);
    for m in &model.state.messages {
        if (m.from == target && m.to == model.state.username.as_deref().unwrap_or(""))
            || (m.from == model.state.username.as_deref().unwrap_or("") && m.to == target)
        {
            let is_own = model.state.username.as_ref().map_or(false, |u| u == &m.from);
            let decrypted_text = if is_own {
                String::from_utf8(m.ciphertext.clone()).unwrap_or_else(|_| "[Ошибка расшифровки]".into())
            } else if let Ok(chat_key) = model.state.get_chat_key(target) {
                if let Ok(pt) = shared::crypto::decrypt_verify(&m.ciphertext, &m.nonce, &m.signature, &chat_key, &m.ciphertext) {
                    String::from_utf8(pt).unwrap_or_else(|_| "[Ошибка кодировки]".into())
                } else {
                    "[Ошибка верификации]".into()
                }
            } else {
                "[Ключи отсутствуют]".into()
            };
            msg_list = msg_list.push(crate::app::component::message::view_bubble(&m.from, &decrypted_text, is_own, m.timestamp));
        }
    }
    column![
        container(row![text(format!("Диалог: {}", target)).size(16)].padding(16))
            .width(Length::Fill)
            .style(styles::top_bar_style()),
        horizontal_rule(1),
        Scrollable::new(msg_list)
            .id(model.scroll_id.clone())
            .height(Length::Fill)
            .width(Length::Fill),
        horizontal_rule(1),
        Space::with_height(10),
        crate::app::component::input::chat_input(
            input,
            "Напишите сообщение...",
            |v| UiMessage::ChatViewInputChanged(v), 
            UiMessage::ChatViewSend             
        )
    ].height(Length::Fill).into()
}

fn view_new_chat<'a>(_model: &'a Model, input: &'a str) -> Element<'a, UiMessage> {
    container(column![\n        text("Создать секретный диалог").size(18), 
        Space::with_height(14), 
        text_input("Введите точный логин пользователя...", input)
            .on_input(UiMessage::NewChatInputChanged)
            .on_submit(UiMessage::NewChatSubmit)
            .padding(12), 
        Space::with_height(16), 
        row![\n            button(text("Отмена").size(13)).padding([8,16]).on_press(UiMessage::NewChatCancel), 
            Space::with_width(Length::Fill), 
            button(text("Открыть чат").size(13))
                .padding([8,16])
                .style(styles::accent_button())
                .on_press(UiMessage::NewChatSubmit)
        ]\n    ].max_width(400).spacing(4))
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn view_empty<'a>() -> Element<'a, UiMessage> {
    container(text("Выберите чат или начните новый секретный диалог").size(14))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}