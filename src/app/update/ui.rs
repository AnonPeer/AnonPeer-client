use iced::Task;
use crate::app::model::Model;
use crate::app::message::UiMessage;
use crate::network::ClientCommand;
use base64::Engine; // <-- ДОБАВЬ ЭТУ СТРОКУ

pub fn toggle_theme(model: &mut Model) -> Task<UiMessage> {
    model.is_light_theme = !model.is_light_theme;
    Task::none()
}

pub fn toggle_hamburger(model: &mut Model) -> Task<UiMessage> {
    model.hamburger_open = !model.hamburger_open;
    if model.hamburger_open {
        model.show_settings = false;
    }
    Task::none()
}

pub fn toggle_settings(model: &mut Model) -> Task<UiMessage> {
    model.show_settings = !model.show_settings;
    if model.show_settings {
        model.hamburger_open = false;
    }
    Task::none()
}

pub fn server_address_changed(model: &mut Model, v: String) -> Task<UiMessage> {
    model.state.server_address = v.clone();
    let _ = model.cmd_tx.send(ClientCommand::UpdateServerAddress(v));
    Task::done(UiMessage::SaveServerAddress)
}

pub fn save_server_address(model: &mut Model) -> Task<UiMessage> {
    let db = model.db.clone();
    let address = model.state.server_address.clone();
    tokio::spawn(async move {
        db.save_server_address(&address).await;
    });
    Task::none()
}

pub fn logout(model: &mut Model) -> Task<UiMessage> {
    model.selected_chat = None;
    model.state.logout();
    let db = model.db.clone();
    tokio::spawn(async move {
        db.clear_local_session().await;
    });
    Task::none()
}

pub fn pick_image(model: &mut Model) -> Task<UiMessage> {
    Task::perform(
        async {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .add_filter("Изображения", &["jpg", "jpeg", "png", "webp"])
                .pick_file()
                .await
            {
                file.path().to_string_lossy().to_string()
            } else {
                String::new()
            }
        },
        |path| UiMessage::ImagePicked(path),
    )
}

pub fn image_picked(model: &mut Model, path: String) -> Task<UiMessage> {
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
                let max_dim = 800;
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
                Ok(("image/jpeg".to_string(), b64))
            })
            .await;
            match result {
                Ok(inner_res) => inner_res,
                Err(_) => Err("Ошибка выполнения задачи".to_string()),
            }
        },
        |res| match res {
            Ok((mime, data)) => UiMessage::ReadyToSendImage(mime, data),
            Err(e) => UiMessage::StatusUpdate(format!("Ошибка: {}", e)),
        },
    )
}

pub fn expand_image(model: &mut Model, id: uuid::Uuid) -> Task<UiMessage> {
    model.expanded_image_id = Some(id);
    Task::none()
}

pub fn close_expanded_image(model: &mut Model) -> Task<UiMessage> {
    model.expanded_image_id = None;
    Task::none()
}

pub fn status_update(model: &mut Model, status: String) -> Task<UiMessage> {
    model.state.status = status;
    Task::none()
}