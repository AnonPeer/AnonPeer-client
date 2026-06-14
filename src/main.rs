use std::env;
use tokio::sync::mpsc;
use tracing_subscriber;
use iced::{Font, Settings};
use std::sync::Arc;

mod domain;
mod infrastructure;
mod network;
mod app;

const NOTO_SANS_BYTES: &[u8] = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");
pub const NOTO_SANS: Font = Font::with_name("Noto Sans");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    
    let db_path = env::var("ANON_DB").unwrap_or_else(|_| "./anon.db".into());
    let db = Arc::new(infrastructure::db::DbManager::new(&db_path).await?);
    
    let keys = db.get_or_generate_keys().await?;
    let server_address = db.get_server_address().await?;
    let saved_session = db.get_saved_session().await?;
    
    let mut state = domain::state::AppState::new_default(
        server_address.clone(), 
        saved_session.clone(), 
        keys
    );
    
    if let Ok(history) = db.load_history().await {
        for msg in history {
            if !state.chats.contains(&msg.from) { state.chats.push(msg.from.clone()); }
            if !state.chats.contains(&msg.to) { state.chats.push(msg.to.clone()); }
            state.messages.push(msg);
        }
        state.chats.sort();
        state.chats.dedup();
    }
    
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let (tx, rx) = mpsc::unbounded_channel();
    
    if let Some((username, session_id)) = &state.saved_session {
        let _ = cmd_tx.send(network::ClientCommand::ValidateSession(session_id.clone()));
        state.username = Some(username.clone());
        state.session_id = Some(session_id.clone());
        state.screen = domain::state::Screen::ChatList;
    }

    let net_state = state.clone();
    let net_tx = tx.clone();
    tokio::spawn(async move {
        if let Err(e) = network::connect(&net_state.server_address, &net_state, net_tx, cmd_rx).await {
            tracing::error!("Network error: {}", e);
        }
    });

    let mut settings = Settings::default();
    settings.fonts.push(std::borrow::Cow::Borrowed(NOTO_SANS_BYTES));
    settings.default_font = NOTO_SANS;

    app::run(state, db, rx, cmd_tx)?;

    Ok(())
}