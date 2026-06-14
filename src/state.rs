use sqlx::SqlitePool;
use shared::errors::AnonError;
use shared::protocol::AppMessage;
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
    pub ed_secret: Vec<u8>,
    pub ed_public: Vec<u8>,
    pub nickname: Option<String>, 
    pub x25519_secret: Vec<u8>,
    pub x25519_public: Vec<u8>,
    pub chats: Vec<String>,
    pub messages: Vec<AppMessage>,
    pub status: String,
    pub db: SqlitePool,
    pub peer_keys: HashMap<String, PeerKeys>,
    pub peer_public_keys: std::collections::HashMap<String, Vec<u8>>,
    pub server_address: String,
    pub saved_session: Option<(String, String)>, 
    pub nickname_cache: std::collections::HashMap<String, String>, 
    
}

impl AppState {
    pub async fn new(db_path: &str) -> Result<Self, AnonError> {
        let clean_path = db_path.trim_start_matches("sqlite://");
        let db_url = format!("sqlite://{}?mode=rwc", clean_path);
        let db = SqlitePool::connect(&db_url).await
            .map_err(|e| AnonError::Db(format!("SQLite: {e}")))?;
        
        sqlx::query("CREATE TABLE IF NOT EXISTS local_keys (id INTEGER PRIMARY KEY, ed_secret BLOB, ed_public BLOB, x25519_secret BLOB, x25519_public BLOB)")
            .execute(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;
        
        sqlx::query("CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, json TEXT, local_order INTEGER)")
            .execute(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;
            
        if let Err(_) = sqlx::query("SELECT local_order FROM messages LIMIT 1").fetch_optional(&db).await {
            let _ = sqlx::query("ALTER TABLE messages ADD COLUMN local_order INTEGER").execute(&db).await;
        }
            
        sqlx::query("CREATE TABLE IF NOT EXISTS server_config (id TEXT PRIMARY KEY, address TEXT)")
            .execute(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;

        sqlx::query("CREATE TABLE IF NOT EXISTS local_session (id INTEGER PRIMARY KEY, username TEXT, session_id TEXT)")
            .execute(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;

        let saved_session: Option<(String, String)> = sqlx::query_as(
            "SELECT username, session_id FROM local_session LIMIT 1"
        ).fetch_optional(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;

        let row: Option<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = sqlx::query_as(
            "SELECT ed_secret, ed_public, x25519_secret, x25519_public FROM local_keys LIMIT 1"
        ).fetch_optional(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;

        let (ed_secret, ed_public, x25519_secret, x25519_public) = match row {
            Some((es, ep, xs, xp)) => (es, ep, xs, xp),
            None => {
                let (es, ep) = shared::crypto::generate_ed25519_keys();
                let (xs, xp) = shared::crypto::generate_x25519_keys();
                sqlx::query("INSERT INTO local_keys (ed_secret, ed_public, x25519_secret, x25519_public) VALUES ($1, $2, $3, $4)")
                    .bind(&es).bind(&ep).bind(&xs).bind(&xp).execute(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;
                (es, ep, xs, xp)
            }
        };

        let saved_address: Option<(String,)> = sqlx::query_as("SELECT address FROM server_config WHERE id = 'current' LIMIT 1")
            .fetch_optional(&db).await.map_err(|e| AnonError::Db(e.to_string()))?;
            
        let server_address = match saved_address {
            Some((addr,)) => addr,
            None => {
                let default_addr = "ws://144.31.215.157:3000/ws".to_string();
                let _ = sqlx::query("INSERT OR REPLACE INTO server_config (id, address) VALUES ('current', $1)")
                    .bind(&default_addr).execute(&db).await;
                default_addr
            }
        };

        let mut state = Self {
            screen: Screen::MainMenu { selection: 0 },
            username: None, session_id: None,
            ed_secret,  ed_public, x25519_secret, x25519_public,
            chats: Vec::new(), messages: Vec::new(), status: String::new(),
            db, 
            peer_keys: HashMap::new(),
            peer_public_keys: HashMap::new(),
            server_address,
            saved_session,
            nickname: None,
            nickname_cache: std::collections::HashMap::new(),
        };

        state.load_history().await?;

        Ok(state)
    }

    pub async fn load_history(&mut self) -> Result<(), AnonError> {
        self.messages.clear();

        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT id, json FROM messages ORDER BY COALESCE(local_order, rowid) ASC"
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| AnonError::Db(e.to_string()))?;

        for (_, json_str) in rows {
            if let Ok(msg) = serde_json::from_str::<AppMessage>(&json_str) {
                self.messages.push(msg.clone());

                if !self.chats.contains(&msg.from) {
                    self.chats.push(msg.from.clone());
                }
                if !self.chats.contains(&msg.to) {
                    self.chats.push(msg.to.clone());
                }
            }
        }
        
        self.chats.sort();
        self.chats.dedup();
        
        Ok(())
    }

    pub fn save_local_session(&self, username: String, session_id: String) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let _ = sqlx::query("INSERT OR REPLACE INTO local_session (id, username, session_id) VALUES (1, $1, $2)")
                .bind(&username).bind(&session_id).execute(&db).await;
        });
    }

    pub fn clear_local_session(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let _ = sqlx::query("DELETE FROM local_session WHERE id = 1").execute(&db).await;
        });
    }

    pub fn logout(&mut self) {
        self.username = None; self.session_id = None; self.chats.clear();
        self.messages.clear(); self.status = "Вы вышли из системы".into();
        self.screen = Screen::MainMenu { selection: 0 };
        self.peer_keys.clear();
        self.peer_public_keys.clear();
    }

    pub fn save_server_address(&self, address: String) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let _ = sqlx::query("INSERT OR REPLACE INTO server_config (id, address) VALUES ('current', $1)")
                .bind(&address)
                .execute(&db)
                .await;
        });
    }

    pub fn cache_peer_keys(&mut self, target: String, ed_pub: Vec<u8>, x_pub: Vec<u8>) {
        self.peer_keys.insert(
            target.clone(),
            PeerKeys {
                ed_public: ed_pub.clone(), 
                x25519_public: x_pub,
            },
        );
        self.peer_public_keys.insert(target, ed_pub);
    }

    pub fn get_chat_key(&self, target: &str) -> Result<Vec<u8>, AnonError> {
        let keys = self.peer_keys.get(target)
            .ok_or_else(|| AnonError::Crypto(format!("Keys for {target} not cached")))?;
        derive_chat_key(&self.x25519_secret, &keys.x25519_public)
    }

    pub async fn save_message(&self, msg: &AppMessage) -> Result<(), AnonError> {
        let mut local_msg = msg.clone();
        local_msg.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let json = serde_json::to_string(&local_msg).map_err(|e| AnonError::Db(e.to_string()))?;
        
        let max_order_row: Option<(Option<i64>,)> = sqlx::query_as("SELECT MAX(local_order) FROM messages")
            .fetch_optional(&self.db).await.map_err(|e| AnonError::Db(e.to_string()))?;
        
        let next_order = match max_order_row {
            Some((Some(max),)) => max + 1,
            _ => 1,
        };

        sqlx::query("INSERT OR IGNORE INTO messages (id, json, local_order) VALUES ($1, $2, $3)")
            .bind(msg.id.to_string()).bind(json).bind(next_order).execute(&self.db).await
            .map_err(|e| AnonError::Db(e.to_string()))?;
        Ok(())
    }

    pub fn add_peer_key(&mut self, username: String, ed_public: Vec<u8>) {
        self.peer_public_keys.insert(username, ed_public);
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






}