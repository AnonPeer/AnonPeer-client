use tokio::sync::mpsc;
use iced::Task;
use iced::widget::scrollable;
use crate::domain::AppState;
use crate::infrastructure::db::DbManager;
use crate::network::{ServerEvent, ClientCommand};
use super::message::UiMessage;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use uuid::Uuid;
use shared::protocol::MessageContent;
use iced::widget::image::Handle as ImageHandle;

pub struct Model {
    pub state: AppState,
    pub db: Arc<DbManager>, 
    pub rx: mpsc::UnboundedReceiver<ServerEvent>,
    pub cmd_tx: mpsc::UnboundedSender<ClientCommand>,
    
    // UI специфичные поля
    pub password_visible: bool,
    pub selected_chat: Option<String>,
    pub scroll_id: scrollable::Id,
    pub hamburger_open: bool,
    pub is_light_theme: bool,
    pub show_settings: bool,
    pub search_query: String,
    pub search_matches: Vec<shared::protocol::UserInfo>,
    
    // Кэши UI
    pub image_cache: Arc<RwLock<HashMap<Uuid, ImageHandle>>>,
    pub decrypted_cache: Arc<RwLock<HashMap<Uuid, Arc<Result<MessageContent, String>>>>>,
    pub expanded_image_id: Option<Uuid>,
}

impl Model {
    pub fn new(state: AppState, db: Arc<DbManager>, rx: mpsc::UnboundedReceiver<ServerEvent>, cmd_tx: mpsc::UnboundedSender<ClientCommand>) -> Self {
        Self {
            state, db, rx, cmd_tx,
            password_visible: false, selected_chat: None,
            scroll_id: scrollable::Id::new("chat_scroll"),
            hamburger_open: false, is_light_theme: false, show_settings: false,
            search_query: String::new(), search_matches: Vec::new(),
            image_cache: Arc::new(RwLock::new(HashMap::new())),
            decrypted_cache: Arc::new(RwLock::new(HashMap::new())),
            expanded_image_id: None,
        }
    }

    // Делегируем весь update в модульную систему
    pub fn update(&mut self, msg: UiMessage) -> Task<UiMessage> {
        super::update::update(self, msg)
    }

    pub fn subscription(&self) -> iced::Subscription<UiMessage> {
        use std::time::Duration;
        iced::time::every(Duration::from_millis(50)).map(|_| UiMessage::Tick)
    }
}