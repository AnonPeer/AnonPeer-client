pub mod message;
pub mod model;
pub mod ui;
pub mod theme;
pub mod component;

use iced::{application, Task, Theme, window};
use tokio::sync::mpsc;
use crate::state::AppState;
use crate::network::{ServerEvent, ClientCommand};
use model::Model;

pub fn run(app: AppState, rx: mpsc::UnboundedReceiver<ServerEvent>, cmd_tx: mpsc::UnboundedSender<ClientCommand>) -> Result<(), iced::Error> {
    let window_settings = window::Settings { ..Default::default() };
    
    application("AnonPeer", Model::update, ui::layout::view)
        .window(window_settings)
        .subscription(|model: &Model| model.subscription())
        .theme(|_| Theme::Dark)
        .run_with(move || (Model::new(app, rx, cmd_tx), Task::none()))
}