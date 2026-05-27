use iced::widget::{button, column, container, row, text, text_input, Column, Space, horizontal_rule, Image, Scrollable};
use iced::{Alignment, Element, Length};
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::app::theme::styles;
use crate::app::component::{sidebar, input, message as message_comp};
use crate::state::Screen;

pub fn view_screen<'a>(model: &'a Model) -> Element<'a, UiMessage> {
    match &model.state.screen {
        Screen::MainMenu { .. } => view_welcome(model),
        Screen::AuthForm { is_register, username, password, .. } => {
            view_auth(model, *is_register, username.as_str(), password.as_str())
        }
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
    const ICON_BYTES: &[u8] = include_bytes!("../../../ico.png");
    let logo = Image::new(iced::widget::image::Handle::from_bytes(ICON_BYTES)).width(Length::Fixed(64.0)).height(Length::Fixed(64.0));
    container(column![logo, Space::with_height(10), text("AnonPeer").size(32), text("Децентрализованный анонимный мессенджер").size(13).style(styles::muted_text()), Space::with_height(30), button(text("Вход в систему").size(14)).width(Length::Fill).padding(12).style(styles::accent_button()).on_press(UiMessage::MainMenuSelect(0)), Space::with_height(10), button(text("Создать новый аккаунт").size(14)).width(Length::Fill).padding(12).style(styles::surface_button()).on_press(UiMessage::MainMenuSelect(1))].align_x(Alignment::Center).max_width(320)).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill).into()
}

fn view_auth<'a>(model: &'a Model, is_reg: bool, user: &'a str, pass: &'a str) -> Element<'a, UiMessage> {
    let title = if is_reg { "Регистрация нового профиля" } else { "Авторизация" };
    container(column![text(title).size(20), Space::with_height(16), text("Имя пользователя").size(12).style(styles::muted_text()), text_input("", user).on_input(UiMessage::AuthUsernameChanged).padding(10), Space::with_height(12), text("Пароль").size(12).style(styles::muted_text()), row![text_input("", pass).on_input(UiMessage::AuthPasswordChanged).secure(!model.password_visible).padding(10), button(text(if model.password_visible { "Скрыть" } else { "Показать" }).size(12)).padding(10).on_press(UiMessage::AuthTogglePasswordVisibility)].spacing(6), Space::with_height(20), row![button(text("Назад").size(14)).padding([10,20]).on_press(UiMessage::AuthBack), Space::with_width(Length::Fill), button(text("Продолжить").size(14)).padding([10,24]).style(styles::accent_button()).on_press(UiMessage::AuthSubmit)]].max_width(360).spacing(4)).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill).into()
}

fn view_chat_view<'a>(model: &'a Model, target: &'a str, input: &'a str) -> Element<'a, UiMessage> {
    let my = model.state.username.as_deref().unwrap_or("");
    
    let filtered: Vec<_> = model.state.messages.iter()
        .filter(|m| !m.ciphertext.is_empty() && ((m.from == my && m.to == target) || (m.to == my && m.from == target)))
        .collect();
    
    let mut msg_list = Column::new().spacing(10).height(Length::Shrink);

    if filtered.is_empty() {
        msg_list = msg_list.push(
            container(text("История сообщений пуста").size(13).style(styles::muted_text()))
                .center_x(Length::Fill)
                .padding(40)
        );
    } else {
        for m in filtered {
            msg_list = msg_list.push(crate::app::component::message::view_bubble(m, &model.state, my));
        }
    }
    
    column![
        row![
            column![
                text(target).size(16),
                text("сквозное шифрование").size(11).style(styles::muted_text())
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
            UiMessage::ChatViewSend             
        )
    ].height(Length::Fill).into()
}

fn view_new_chat<'a>(_model: &'a Model, input: &'a str) -> Element<'a, UiMessage> {
    container(column![text("Создать секретный диалог").size(18), Space::with_height(14), text_input("Введите точный логин пользователя...", input).on_input(UiMessage::NewChatInputChanged).on_submit(UiMessage::NewChatSubmit).padding(12), Space::with_height(16), row![button(text("Отмена").size(13)).padding([8,16]).on_press(UiMessage::NewChatCancel), Space::with_width(Length::Fill), button(text("Открыть чат").size(13)).padding([8,16]).style(styles::accent_button()).on_press(UiMessage::NewChatSubmit)]].max_width(400).spacing(4)).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill).into()
}

fn view_empty<'a>() -> Element<'a, UiMessage> {
    container(text("Выберите чат или создайте новый для начала общения").size(14).style(styles::muted_text())).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill).into()
}