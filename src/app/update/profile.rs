use iced::Task;
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::network::ClientCommand;
use base64::Engine;

pub fn view(model: &mut Model, username: String) -> Task<UiMessage> {
    model.state.screen = crate::domain::state::Screen::UserProfile { username: username.clone() };
    let _ = model.cmd_tx.send(ClientCommand::RequestProfile(username));
    Task::none()
}

pub fn received(model: &mut Model, user: shared::protocol::UserInfo) -> Task<UiMessage> {
    model.state.cache_profile(&user);
    Task::none()
}

pub fn close(model: &mut Model) -> Task<UiMessage> {
    model.state.screen = crate::domain::state::Screen::ChatList;
    Task::none()
}

pub fn open_edit(model: &mut Model) -> Task<UiMessage> {
    let bio = model.state.username.as_ref()
        .and_then(|u| model.state.bio_cache.get(u).cloned())
        .unwrap_or_default();
    let avatar = model.state.username.as_ref()
        .and_then(|u| model.state.avatar_cache.get(u).cloned())
        .flatten();
    model.state.edit_bio = bio;
    model.state.edit_avatar_base64 = avatar;
    model.state.screen = crate::domain::state::Screen::EditProfile;
    Task::none()
}

pub fn bio_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    model.state.edit_bio = v;
    Task::none()
}

pub fn avatar_picked(model: &mut Model, path: String) -> Task<UiMessage> {
    if path.is_empty() {
        return Task::none();
    }
    Task::perform(
        async move {
            let result = tokio::task::spawn_blocking(move || {
                let img = match image::open(&path) {
                    Ok(i) => i,
                    Err(_) => return Err("Не удалось открыть изображение".to_string()),
                };
                let max_dim = 256;
                let (w, h) = (img.width(), img.height());
                let (new_w, new_h) = if w > max_dim || h > max_dim {
                    if w > h {
                        (max_dim, (h as f32 / w as f32 * max_dim as f32) as u32)
                    } else {
                        ((w as f32 / h as f32 * max_dim as f32) as u32, max_dim)
                    }
                } else {
                    (w, h)
                };
                let resized = img.resize(new_w, new_h, image::imageops::FilterType::Triangle);
                let mut buffer = std::io::Cursor::new(Vec::new());
                resized
                    .write_to(&mut buffer, image::ImageFormat::Jpeg)
                    .map_err(|_| "Ошибка сжатия".to_string())?;
                let bytes = buffer.into_inner();
                let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                Ok(b64)
            })
            .await;
            match result {
                Ok(inner) => inner,
                Err(_) => Err("Ошибка задачи".to_string()),
            }
        },
        |res| match res {
            Ok(b64) => UiMessage::EditProfileAvatarReady(b64),
            Err(e) => UiMessage::StatusUpdate(format!("Ошибка: {}", e)),
        },
    )
}

pub fn avatar_ready(model: &mut Model, b64: String) -> Task<UiMessage> {
    model.state.edit_avatar_base64 = Some(b64);
    Task::none()
}

pub fn save(model: &mut Model) -> Task<UiMessage> {
    let bio = model.state.edit_bio.clone();
    let avatar = model.state.edit_avatar_base64.clone();
    let _ = model.cmd_tx.send(ClientCommand::UpdateProfile {
        bio: Some(bio.clone()),
        avatar_base64: avatar.clone(),
    });
    if let Some(ref username) = model.state.username.clone() {
        model.state.bio_cache.insert(username.clone(), bio);
        model.state.avatar_cache.insert(username.clone(), avatar);
    }
    model.state.screen = crate::domain::state::Screen::ChatList;
    model.state.status = "Профиль обновлён".into();
    Task::none()
}

pub fn updated(model: &mut Model, user: shared::protocol::UserInfo) -> Task<UiMessage> {
    model.state.cache_profile(&user);
    Task::none()
}