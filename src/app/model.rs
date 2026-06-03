use tokio::sync::mpsc;
use iced::Task;
use iced::widget::scrollable;
use crate::state::AppState;
use crate::network::{ServerEvent, ClientCommand};
use super::message::UiMessage;

pub struct Model {
    pub state: AppState,
    pub rx: mpsc::UnboundedReceiver<ServerEvent>,
    pub cmd_tx: mpsc::UnboundedSender<ClientCommand>,
    pub password_visible: bool,
    pub selected_chat: Option<String>,
    pub scroll_id: scrollable::Id,
    pub hamburger_open: bool,
    pub is_light_theme: bool,
    pub show_settings: bool,
    pub search_found: bool,
    pub search_result: Option<(String, bool)>,
    pub search_query: String,
    pub search_matches: Vec<String>,
}

impl Model {
    pub fn new(state: AppState, rx: mpsc::UnboundedReceiver<ServerEvent>, cmd_tx: mpsc::UnboundedSender<ClientCommand>) -> Self {
        let mut initial_state = state;

        if let Some((username, session_id)) = &initial_state.saved_session {
            let _ = cmd_tx.send(ClientCommand::ValidateSession(session_id.clone()));
            initial_state.username = Some(username.clone());
            initial_state.session_id = Some(session_id.clone());
            initial_state.screen = crate::state::Screen::ChatList;
        }

        Self {
            state: initial_state,
            rx,
            cmd_tx,
            password_visible: false,
            selected_chat: None,
            scroll_id: scrollable::Id::new("chat_scroll"),
            hamburger_open: false,
            is_light_theme: false,
            show_settings: false,
            search_query: String::new(),
            search_found: false,
            search_result: None,
            search_matches: Vec::new(),
        }
    }

    pub fn update(&mut self, msg: UiMessage) -> Task<UiMessage> {
        use UiMessage::*;
        match msg {
            Tick => self.poll_network(),
            Network(ev) => self.handle_network(ev),
            MainMenuSelect(idx) => {
                self.state.screen = crate::state::Screen::AuthForm { 
                    is_register: idx == 1, 
                    username: String::new(), 
                    password: String::new(), 
                    focused: crate::state::FocusedField::Username 
                };
                self.state.status.clear(); 
                Task::none()
            }
            AuthUsernameChanged(v) => { 
                if let crate::state::Screen::AuthForm { username, .. } = &mut self.state.screen { 
                    *username = v; 
                } 
                Task::none() 
            }
            AuthPasswordChanged(v) => { 
                if let crate::state::Screen::AuthForm { password, .. } = &mut self.state.screen { 
                    *password = v; 
                } 
                Task::none() 
            }
            AuthSubmit => self.auth_submit(),
            AuthTogglePasswordVisibility => { 
                self.password_visible = !self.password_visible; 
                Task::none() 
            }
            AuthBack => { 
                self.state.screen = crate::state::Screen::MainMenu { selection: 0 }; 
                self.state.status.clear(); 
                Task::none() 
            }
            
        ChatSelected(t) => {
            if !self.state.chats.contains(&t) && self.state.username.as_ref().map_or(true, |u| u != &t) {
                self.state.chats.push(t.clone());
                self.state.chats.sort();
            }

            self.selected_chat = Some(t.clone());
            self.state.screen = crate::state::Screen::ChatView { 
                target: t.clone(), 
                input: String::new() 
            };
            self.state.status.clear();
            if !self.state.peer_keys.contains_key(&t) { 
                let _ = self.cmd_tx.send(ClientCommand::FetchPeerKeys(t)); 
            }
            scrollable::scroll_to(self.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX })
        }

            OpenNewChatScreen => { 
                self.state.screen = crate::state::Screen::NewChat { input: String::new() }; 
                self.state.status.clear(); 
                Task::none() 
            }
            NewChatInputChanged(v) => { 
                if let crate::state::Screen::NewChat { input } = &mut self.state.screen { 
                    *input = v; 
                } 
                Task::none() 
            }
            NewChatSubmit => self.new_chat_submit(),
            NewChatCancel => { 
                self.state.screen = crate::state::Screen::ChatList; 
                self.state.status.clear(); 
                Task::none() 
            }
            ChatViewInputChanged(v) => { 
                if let crate::state::Screen::ChatView { input, .. } = &mut self.state.screen { 
                    *input = v; 
                } 
                Task::none() 
            }
            ChatViewSend => {
                self.chat_view_send();
                scrollable::scroll_to(self.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX })
            }
            Logout => { 
                self.selected_chat = None; 
                self.state.logout(); 
                self.state.clear_local_session(); 
                Task::none() 
            }
            FetchKeysFor(t) => { 
                let _ = self.cmd_tx.send(ClientCommand::FetchPeerKeys(t)); 
                Task::none() 
            }
            KeysReceived { target, ed_public, x25519_public } => { 
                self.state.cache_peer_keys(target, ed_public, x25519_public); 
                Task::none() 
            }
            ToggleHamburgerMenu => {
                self.hamburger_open = !self.hamburger_open;
                if self.hamburger_open { self.show_settings = false; }
                Task::none()
            }
            ToggleTheme => {
                self.is_light_theme = !self.is_light_theme;
                Task::none()
            }
            ToggleSettings => {
                self.show_settings = !self.show_settings;
                if self.show_settings { self.hamburger_open = false; } 
                Task::none()
            }
            ServerAddressChanged(v) => {
                self.state.server_address = v.clone();
                let _ = self.cmd_tx.send(ClientCommand::UpdateServerAddress(v));
                Task::done(SaveServerAddress)
            }
            SaveServerAddress => {
                let current_state = self.state.clone();
                let address = self.state.server_address.clone();
                tokio::spawn(async move {
                    current_state.save_server_address(address.clone());
                });
                Task::none()
            }

            UiMessage::SearchInputChanged(v) => {
                self.search_query = v.clone();
                if v.len() >= 2 {
                    let _ = self.cmd_tx.send(ClientCommand::SearchPrefix(v));
                } else {
                    self.search_matches.clear();
                }
                Task::none()
            }
            UiMessage::SearchResultSelected(name) => {
                self.search_query.clear();
                self.search_matches.clear();
                return Task::done(UiMessage::ChatSelected(name));
            }
            UiMessage::SearchResultsReceived(matches) => {
                self.search_matches = matches;
                Task::none()
            }

            UiMessage::SearchSubmit => {
                if !self.search_query.trim().is_empty() {
                    let _ = self.cmd_tx.send(ClientCommand::SearchPrefix(self.search_query.clone()));
                }
                Task::none()
            }

            UiMessage::SearchResult { username, exists } => {
                if exists {
                    self.search_query.clear();
                    self.search_matches.clear();
                    return Task::done(UiMessage::ChatSelected(username));
                }
                Task::none()
            }
        }
    }

    fn poll_network(&mut self) -> Task<UiMessage> {
        while let Ok(ev) = self.rx.try_recv() {
            return Task::perform(async move { ev }, UiMessage::Network);
        }
        Task::none()
    }

    fn handle_network(&mut self, ev: ServerEvent) -> Task<UiMessage> {
        use ServerEvent::*;
        match ev {
            AuthOk(sid) => {
                self.state.session_id = Some(sid.clone());
                
                if let crate::state::Screen::AuthForm { username, .. } = &self.state.screen { 
                    self.state.username = Some(username.clone()); 
                    self.state.save_local_session(username.clone(), sid.clone());
                } else if let Some((saved_user, _)) = &self.state.saved_session {
                    self.state.username = Some(saved_user.clone());
                }
                
                self.state.screen = crate::state::Screen::ChatList; 
                self.state.status = "Успешная авторизация".into();
                Task::none()
            }
            AuthErr(e) => {
                self.state.status = format!("Ошибка: {}", e);
                if self.state.saved_session.is_some() {
                    self.state.saved_session = None;
                    self.state.clear_local_session();
                    self.state.screen = crate::state::Screen::MainMenu { selection: 0 };
                    self.state.username = None;
                    self.state.session_id = None;
                }
                Task::none()
            }
            UserSearchResult { username, exists } => Task::done(UiMessage::SearchResult { username, exists }),
            NewMessage(mut msg) => {
                // Прямо здесь форсируем локальное время устройства для отображения
                msg.timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                self.state.messages.push(msg.clone());
                let db = self.state.clone(); 
                let m = msg.clone();
                tokio::spawn(async move { 
                     let _ = db.save_message(&m).await; 
                });
                if !self.state.chats.contains(&msg.from) && self.state.username.as_ref().map_or(true, |u| u != &msg.from) {
                    self.state.chats.push(msg.from.clone()); 
                    self.state.chats.sort();
                }
                if !self.state.peer_keys.contains_key(&msg.from) { 
                    let _ = self.cmd_tx.send(ClientCommand::FetchPeerKeys(msg.from)); 
                }
                scrollable::scroll_to(self.scroll_id.clone(), scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX })
            }
            Disconnect => {
                self.state.status = "Соединение разорвано".into();
                Task::none()
            }
            PeerKeys { target, ed_public, x25519_public } => {
                self.state.cache_peer_keys(target.clone(), ed_public.clone(), x25519_public.clone());
                Task::done(UiMessage::KeysReceived { target, ed_public, x25519_public })
            }
            SearchResults(matches) => {
                Task::done(UiMessage::SearchResultsReceived(matches))
            }
        }
    }

    fn auth_submit(&mut self) -> Task<UiMessage> {
        if let crate::state::Screen::AuthForm { is_register, username, password, .. } = &self.state.screen {
            if !username.is_empty() && !password.is_empty() {
                let cmd = if *is_register {
                    ClientCommand::Register(
                        username.clone(), 
                        password.clone(), 
                        self.state.ed_public.clone(), 
                        self.state.x25519_public.clone()
                    )
                } else {
                    ClientCommand::Login(
                        username.clone(), 
                        password.clone(), 
                        self.state.ed_public.clone(), 
                        self.state.x25519_public.clone()
                    )
                };
                let _ = self.cmd_tx.send(cmd);
                self.state.status = "Запрос отправлен...".into();
            }
        }
        Task::none()
    }

    fn new_chat_submit(&mut self) -> Task<UiMessage> {
        if let crate::state::Screen::NewChat { input } = &self.state.screen {
            let target = input.trim().to_string();
            if !target.is_empty() {
                if !self.state.chats.contains(&target) && self.state.username.as_ref().map_or(true, |u| u != &target) {
                    self.state.chats.push(target.clone()); 
                    self.state.chats.sort();
                }
                self.selected_chat = Some(target.clone());
                self.state.screen = crate::state::Screen::ChatView { 
                    target: target.clone(), 
                    input: String::new() 
                };
                if !self.state.peer_keys.contains_key(&target) { 
                    let _ = self.cmd_tx.send(ClientCommand::FetchPeerKeys(target)); 
                }
            }
        }
        Task::none()
    }

    fn chat_view_send(&mut self) {
        if let crate::state::Screen::ChatView { target, input } = &self.state.screen {
            if !input.is_empty() {
                if let Some(user) = &self.state.username {
                    if let Ok(chat_key) = self.state.get_chat_key(target) {
                        let pt = input.as_bytes();
                        if let Ok((ct, nonce, sig, _eph_public)) = shared::crypto::encrypt_sign(
                            pt, 
                            &chat_key, 
                            &self.state.ed_secret
                        ) {
                            
                            let msg = shared::protocol::AppMessage {
                                id: uuid::Uuid::new_v4(), 
                                from: user.clone(), 
                                to: target.clone(),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs(), 
                                ciphertext: ct, 
                                nonce, 
                                signature: sig, 
                                salt: vec![],
                            };

                            self.state.messages.push(msg.clone());
                            let db = self.state.clone(); 
                            let m = msg.clone();
                            tokio::spawn(async move { 
                                let _ = db.save_message(&m).await; 
                            });
                            let _ = self.cmd_tx.send(ClientCommand::Send(msg));
                            if let crate::state::Screen::ChatView { input, .. } = &mut self.state.screen { 
                                *input = String::new(); 
                            }
                        } else { 
                            self.state.status = "Ошибка шифрования".into(); 
                        }
                    } else {
                        self.state.status = "Нет ключей собеседника".into();
                        let _ = self.cmd_tx.send(ClientCommand::FetchPeerKeys(target.clone()));
                    }
                }
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<UiMessage> {
        use std::time::Duration;
        iced::time::every(Duration::from_millis(50)).map(|_| UiMessage::Tick)
    }
}