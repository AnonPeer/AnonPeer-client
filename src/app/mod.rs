pub mod message;
pub mod model;
pub mod ui;
pub mod theme;
pub mod component;
pub mod update; 

use iced::{application, Task, Theme, window};
use tokio::sync::mpsc;
use std::sync::Arc;
use crate::domain::state::AppState;
use crate::infrastructure::db::DbManager;
use crate::network::{ServerEvent, ClientCommand};
use model::Model;

pub fn run(
    app: AppState, 
    db: Arc<DbManager>, 
    rx: mpsc::UnboundedReceiver<ServerEvent>, 
    cmd_tx: mpsc::UnboundedSender<ClientCommand>
) -> Result<(), iced::Error> {
    let window_settings = window::Settings { ..Default::default() };
    application("AnonPeer", Model::update, ui::layout::view)
        .window(window_settings)
        .subscription(|model: &Model| model.subscription())
        .theme(|model: &Model| if model.is_light_theme { Theme::Light } else { Theme::Dark })
        .run_with(move || (Model::new(app, db, rx, cmd_tx), Task::none())) 
}