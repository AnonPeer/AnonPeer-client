use std::env;
use tokio::sync::mpsc;
use tracing_subscriber;
use crate::state::AppState;
use crate::network::{ClientCommand, ServerEvent};
use iced::{Font, Settings};

mod state;
mod network;
mod app;

const APP_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/NotoSansSymbols-VariableFont_wght.ttf");
pub const APP_FONT: Font = Font::with_name("Noto Sans");

const EMOJI_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/NotoColorEmoji.ttf");
pub const EMOJI_FONT: Font = Font::with_name("Noto Color Emoji");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    
    let db_path = env::var("ANON_DB").unwrap_or_else(|_| "./anon.db".into());
    let app = AppState::new(&db_path).await?;
    let ws_url = env::var("ANON_SERVER").unwrap_or_else(|_| app.server_address.clone());

    let (tx, rx) = mpsc::unbounded_channel::<ServerEvent>();
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<ClientCommand>();

    let app_net = app.clone();
    let tx_net = tx.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::network::connect(&ws_url, &app_net, tx_net, cmd_rx).await {
            tracing::error!("Network error: {}", e);
        }
    });

    let mut settings = Settings::default();
    settings.fonts.push(std::borrow::Cow::Borrowed(APP_FONT_BYTES));
    settings.fonts.push(std::borrow::Cow::Borrowed(EMOJI_FONT_BYTES));
    settings.default_font = APP_FONT;

    app::run(app, rx, cmd_tx)?;

    Ok(())
}