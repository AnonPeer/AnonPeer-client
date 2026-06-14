use shared::errors::AnonError;
use shared::protocol::{AppMessage, UserInfo};
use shared::crypto::derive_chat_key;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum FocusedField { Username, Password }

#[derive(Clone, Debug)]
pub enum Screen {
    MainMenu { selection: usize },
    AuthForm { is_register: bool, username: String, nickname: String, password: String, focused: FocusedField },
    ChatList,
    NewChat { input: String },
    ChatView { target: String, input: String },
    UserProfile { username: String },
    EditProfile,
}

#[derive(Clone, Debug)]
pub struct PeerKeys {
    pub ed_public: Vec<u8>,
    pub x25519_public: Vec<u8>,
}

#[derive(Clone)]
pub struct AppState {
    pub screen: Screen,
    pub username: Option<String>,
    pub session_id: Option<String>,
    pub nickname: Option<String>,
    
    // Криптография
    pub ed_secret: Vec<u8>,
    pub ed_public: Vec<u8>,
    pub x25519_secret: Vec<u8>,
    pub x25519_public: Vec<u8>,
    
    // Данные чатов
    pub chats: Vec<String>,
    pub messages: Vec<AppMessage>,
    pub status: String,
    
    // Кэши
    pub peer_keys: HashMap<String, PeerKeys>,
    pub peer_public_keys: HashMap<String, Vec<u8>>,
    pub nickname_cache: HashMap<String, String>,
    pub bio_cache: HashMap<String, String>,
    pub avatar_cache: HashMap<String, Option<String>>,
    pub last_seen_cache: HashMap<String, Option<u64>>,
    
    // Настройки
    pub server_address: String,
    pub saved_session: Option<(String, String)>,
    
    // Временные данные для форм
    pub edit_bio: String,
    pub edit_avatar_base64: Option<String>,
}

impl AppState {
    pub fn new_default(server_address: String, saved_session: Option<(String, String)>, keys: (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)) -> Self {
        Self {
            screen: Screen::MainMenu { selection: 0 },
            username: None, session_id: None, nickname: None,
            ed_secret: keys.0, ed_public: keys.1, x25519_secret: keys.2, x25519_public: keys.3,
            chats: Vec::new(), messages: Vec::new(), status: String::new(),
            peer_keys: HashMap::new(), peer_public_keys: HashMap::new(),
            nickname_cache: HashMap::new(), bio_cache: HashMap::new(),
            avatar_cache: HashMap::new(), last_seen_cache: HashMap::new(),
            server_address, saved_session,
            edit_bio: String::new(), edit_avatar_base64: None,
        }
    }

    pub fn cache_peer_keys(&mut self, target: String, ed_pub: Vec<u8>, x_pub: Vec<u8>) {
        self.peer_keys.insert(target.clone(), PeerKeys { ed_public: ed_pub.clone(), x25519_public: x_pub });
        self.peer_public_keys.insert(target, ed_pub);
    }

    pub fn get_chat_key(&self, target: &str) -> Result<Vec<u8>, AnonError> {
        let keys = self.peer_keys.get(target)
            .ok_or_else(|| AnonError::Crypto(format!("Keys for {target} not cached")))?;
        derive_chat_key(&self.x25519_secret, &keys.x25519_public)
    }

    pub fn get_sender_public_key(&self, sender: &str) -> Option<&[u8]> {
        if self.username.as_deref() == Some(sender) {
            return Some(&self.ed_public);
        }
        self.peer_public_keys.get(sender).map(|k| k.as_slice())
    }

    pub fn get_sas_code(&self, target: &str) -> Option<String> {
        let keys = self.peer_keys.get(target)?;
        shared::crypto::generate_sas(&self.x25519_secret, &keys.x25519_public).ok()
    }

    pub fn cache_profile(&mut self, user: &UserInfo) {
        self.nickname_cache.insert(user.username.clone(), user.nickname.clone());
        self.bio_cache.insert(user.username.clone(), user.bio.clone());
        self.avatar_cache.insert(user.username.clone(), user.avatar_base64.clone());
        self.last_seen_cache.insert(user.username.clone(), user.last_seen);
        
        if let Some(ref domain) = user.server_domain {
            let key = format!("{}@{}", user.username, domain);
            self.nickname_cache.insert(key.clone(), user.nickname.clone());
            self.bio_cache.insert(key.clone(), user.bio.clone());
            self.avatar_cache.insert(key.clone(), user.avatar_base64.clone());
            self.last_seen_cache.insert(key, user.last_seen);
        }
    }

    pub fn logout(&mut self) {
        self.username = None; self.session_id = None; self.nickname = None;
        self.chats.clear(); self.messages.clear(); self.status = "Вы вышли из системы".into();
        self.screen = Screen::MainMenu { selection: 0 };
        self.peer_keys.clear(); self.peer_public_keys.clear();
    }
}