use iced::Task;
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::network::ClientCommand;

pub fn nickname_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    if let crate::domain::state::Screen::AuthForm { nickname, .. } = &mut model.state.screen {
        *nickname = v;
    }
    Task::none()
}

pub fn username_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    if let crate::domain::state::Screen::AuthForm { username, .. } = &mut model.state.screen {
        *username = v;
    }
    Task::none()
}

pub fn password_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    if let crate::domain::state::Screen::AuthForm { password, .. } = &mut model.state.screen {
        *password = v;
    }
    Task::none()
}

pub fn toggle_password(model: &mut Model) -> Task<UiMessage> {
    model.password_visible = !model.password_visible;
    Task::none()
}

pub fn back(model: &mut Model) -> Task<UiMessage> {
    model.state.screen = crate::domain::state::Screen::MainMenu { selection: 0 };
    model.state.status.clear();
    Task::none()
}

pub fn main_menu_select(model: &mut Model, idx: usize) -> Task<UiMessage> {
    model.state.screen = crate::domain::state::Screen::AuthForm {
        is_register: idx == 1,
        nickname: String::new(),
        username: String::new(),
        password: String::new(),
        focused: crate::domain::state::FocusedField::Username,
    };
    model.state.status.clear();
    Task::none()
}

pub fn submit(model: &mut Model) -> Task<UiMessage> {
    if let crate::domain::state::Screen::AuthForm { is_register, nickname, username, password, .. } = &model.state.screen {
        if username.trim().is_empty() || password.trim().is_empty() || (*is_register && nickname.trim().is_empty()) {
            model.state.status = "Заполните все поля!".into();
            return Task::none();
        }

        let cmd = if *is_register {
            ClientCommand::Register(
                nickname.clone(),
                username.clone(),
                password.clone(),
                model.state.ed_public.clone(),
                model.state.x25519_public.clone(),
            )
        } else {
            ClientCommand::Login(
                username.clone(),
                password.clone(),
                model.state.ed_public.clone(),
                model.state.x25519_public.clone(),
            )
        };
        let _ = model.cmd_tx.send(cmd);
        model.state.status = "Запрос отправлен...".into();
    }
    Task::none()
}