use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use shared::errors::AnonError;
use shared::protocol::{ClientPayload, ServerPayload, AppMessage};

#[derive(Debug, Clone)]
pub enum ServerEvent {
    AuthOk(String),
    AuthErr(String),
    NewMessage(AppMessage),
    PeerKeys { target: String, ed_public: Vec<u8>, x25519_public: Vec<u8> },
    Disconnect,
}

#[derive(Debug, Clone)]
pub enum ClientCommand {
    Register(String, String, Vec<u8>, Vec<u8>),
    Login(String, String, Vec<u8>, Vec<u8>),
    Send(AppMessage),
    FetchPeerKeys(String),
}

pub async fn connect(
    url: &str,
    _state: &crate::state::AppState,
    tx: mpsc::UnboundedSender<ServerEvent>,
    mut cmd_rx: mpsc::UnboundedReceiver<ClientCommand>
) -> Result<(), AnonError> {
    let (ws_stream, _) = connect_async(url).await
        .map_err(|e| AnonError::Network(format!("WS Connect: {e}")))?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let tx_clone = tx.clone();

    let recv_task = tokio::spawn(async move {
        println!("[СЕТЬ] Поток чтения запущен.");
        while let Some(res) = ws_receiver.next().await {
            match res {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        match serde_json::from_str::<ServerPayload>(&text) {
                            Ok(payload) => match payload {
                                ServerPayload::Forward { msg: app_msg } => {
                                    let _ = tx_clone.send(ServerEvent::NewMessage(app_msg));
                                }
                                ServerPayload::AuthOk { session_id } => {
                                    let _ = tx_clone.send(ServerEvent::AuthOk(session_id));
                                }
                                ServerPayload::AuthErr(e) => {
                                    let _ = tx_clone.send(ServerEvent::AuthErr(e));
                                }
                                ServerPayload::PeerKeys { target, ed_public, x25519_public } => {
                                    let _ = tx_clone.send(ServerEvent::PeerKeys { target, ed_public, x25519_public });
                                }
                            },
                            Err(err) => println!("[ОШИБКА ПАРСИНГА] {:?}", err),
                        }
                    }
                }
                Err(e) => {
                    println!("[СЕТЬ ОШИБКА] Соединение разорвано: {}", e);
                    let _ = tx_clone.send(ServerEvent::Disconnect);
                    break;
                }
            }
        }
    });

    let send_task = tokio::spawn(async move {
        while let Some(cmd) = cmd_rx.recv().await {
            let payload = match cmd {
                ClientCommand::Register(u, p, ep, xp) => 
                    ClientPayload::Register { username: u, password: p, ed_public: ep, x25519_public: xp },
                ClientCommand::Login(u, p, ep, xp) => 
                    ClientPayload::Login { username: u, password: p, ed_public: ep, x25519_public: xp },
                ClientCommand::Send(msg) => ClientPayload::SendMessage { msg },
                ClientCommand::FetchPeerKeys(t) => ClientPayload::RequestKeys { target: t },
            };
            if let Ok(json) = serde_json::to_string(&payload) {
                if ws_sender.send(Message::Text(json)).await.is_err() { break; }
            }
        }
    });

    tokio::select! {
        _ = recv_task => {},
        _ = send_task => {},
    }
    Ok(())
}