use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use shared::errors::AnonError;
use shared::protocol::{ClientPayload, ServerPayload, AppMessage};

#[derive(Debug, Clone)]
pub enum ServerEvent {
    AuthOk(String),
    AuthErr(String),
    NewMessage(AppMessage),
    PeerKeys { target: String, ed_public: Vec<u8>, x25519_public: Vec<u8> },
    Disconnect,
    UserSearchResult { username: String, exists: bool },
    SearchResults(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum ClientCommand {
    Register(String, String, Vec<u8>, Vec<u8>),
    Login(String, String, Vec<u8>, Vec<u8>),
    Send(AppMessage),
    FetchPeerKeys(String),
    UpdateServerAddress(String),
    SearchUser(String),
    SearchPrefix(String),
    ValidateSession(String),
}

pub async fn connect(
    initial_url: &str,
    _state: &crate::state::AppState,
    tx: mpsc::UnboundedSender<ServerEvent>,
    mut cmd_rx: mpsc::UnboundedReceiver<ClientCommand>
) -> Result<(), AnonError> {
    let mut current_url = initial_url.to_string();

    loop {
        let (ws_stream, _) = match connect_async(&current_url).await {
            Ok(v) => v,
            Err(_) => {
                let _ = tx.send(ServerEvent::Disconnect);
                if let Some(cmd) = cmd_rx.recv().await {
                    match cmd {
                        ClientCommand::UpdateServerAddress(new_url) => {
                            current_url = new_url;
                            continue;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                break;
            }
        };

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let tx_clone = tx.clone();
        let (internal_tx, mut internal_rx) = mpsc::unbounded_channel::<ClientCommand>();

        let receive_task = tokio::spawn(async move {
            while let Some(msg_res) = ws_receiver.next().await {
                match msg_res {
                    Ok(WsMessage::Text(text)) => {
                        if let Ok(payload) = serde_json::from_str::<ServerPayload>(&text) {
                            match payload {
                                ServerPayload::AuthOk { session_id } => {
                                    let _ = tx_clone.send(ServerEvent::AuthOk(session_id));
                                }
                                ServerPayload::AuthErr(reason) => {
                                    let _ = tx_clone.send(ServerEvent::AuthErr(reason));
                                }

                                ServerPayload::Forward { msg } => {
                                    let _ = tx_clone.send(ServerEvent::NewMessage(msg));
                                }
                                ServerPayload::PeerKeys { target, ed_public, x25519_public } => {
                                    let _ = tx_clone.send(ServerEvent::PeerKeys { target, ed_public, x25519_public });
                                }


                                ServerPayload::UserSearchResult { username, exists } => {
                                    let _ = tx_clone.send(ServerEvent::UserSearchResult { username, exists });
                                }


                                ServerPayload::SearchResults { matches } => {
                                    let _ = tx_clone.send(ServerEvent::SearchResults(matches));
                                }


                            }

                        }
                    }
                    Ok(WsMessage::Close(_)) | Err(_) => {
                        let _ = tx_clone.send(ServerEvent::Disconnect);
                        break;
                    }
                    _ => {}
                }
            }
        });

        let mut changed_url = None;

        let send_task = tokio::spawn(async move {
            while let Some(cmd) = internal_rx.recv().await {
                let payload = match cmd {
                    ClientCommand::Register(u, p, ep, xp) => 
                        ClientPayload::Register { username: u, password: p, ed_public: ep, x25519_public: xp },
                    ClientCommand::Login(u, p, ep, xp) => 
                        ClientPayload::Login { username: u, password: p, ed_public: ep, x25519_public: xp },
                    ClientCommand::Send(msg) => ClientPayload::SendMessage { msg },
                    ClientCommand::FetchPeerKeys(t) => ClientPayload::RequestKeys { target: t },
                    ClientCommand::UpdateServerAddress(_) => unreachable!(),
                    ClientCommand::SearchUser(u) => ClientPayload::SearchUser { username: u },
                    ClientCommand::SearchPrefix(p) => ClientPayload::SearchPrefix { prefix: p }, <--
                    ClientCommand::ValidateSession(session_id) => ClientPayload::ValidateSession { session_id }, 
                };
                if let Ok(json) = serde_json::to_string(&payload) {
                    if ws_sender.send(WsMessage::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
            let _ = ws_sender.close().await;
        });

        while let Some(cmd) = cmd_rx.recv().await {
            match cmd {
                ClientCommand::UpdateServerAddress(new_url) => {
                    changed_url = Some(new_url);
                    break;
                }
                other => {
                    if internal_tx.send(other).is_err() {
                        break;
                    }
                }
            }
        }

        receive_task.abort();
        send_task.abort();

        if let Some(new_url) = changed_url {
            current_url = new_url;
        } else {
            break;
        }
    }

    Ok(())
}
