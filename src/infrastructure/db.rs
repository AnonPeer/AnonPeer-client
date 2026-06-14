use sqlx::SqlitePool;
use shared::errors::AnonError;
use shared::protocol::AppMessage;

pub struct DbManager {
    pub pool: SqlitePool,
}

impl DbManager {
    pub async fn new(db_path: &str) -> Result<Self, AnonError> {
        let clean_path = db_path.trim_start_matches("sqlite://");
        let db_url = format!("sqlite://{}?mode=rwc", clean_path);
        let pool = SqlitePool::connect(&db_url).await
            .map_err(|e| AnonError::Db(format!("SQLite connect: {e}")))?;

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS local_keys (id INTEGER PRIMARY KEY, ed_secret BLOB, ed_public BLOB, x25519_secret BLOB, x25519_public BLOB)").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, json TEXT, local_order INTEGER)").execute(&pool).await;
        let _ = sqlx::query("ALTER TABLE messages ADD COLUMN local_order INTEGER").execute(&pool).await; 
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS server_config (id TEXT PRIMARY KEY, address TEXT)").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS local_session (id INTEGER PRIMARY KEY, username TEXT, session_id TEXT)").execute(&pool).await;

        Ok(Self { pool })
    }

    pub async fn load_history(&self) -> Result<Vec<AppMessage>, AnonError> {
        let rows: Vec<(String, String)> = sqlx::query_as("SELECT id, json FROM messages ORDER BY COALESCE(local_order, rowid) ASC")
            .fetch_all(&self.pool).await.map_err(|e| AnonError::Db(e.to_string()))?;
        
        let mut msgs = Vec::new();
        for (_, json_str) in rows {
            if let Ok(msg) = serde_json::from_str::<AppMessage>(&json_str) {
                msgs.push(msg);
            }
        }
        Ok(msgs)
    }

    pub async fn save_message(&self, msg: &AppMessage) -> Result<(), AnonError> {
        let mut local_msg = msg.clone();
        local_msg.timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        let json = serde_json::to_string(&local_msg).map_err(|e| AnonError::Db(e.to_string()))?;
        
        let max_order: Option<(Option<i64>,)> = sqlx::query_as("SELECT MAX(local_order) FROM messages").fetch_optional(&self.pool).await.map_err(|e| AnonError::Db(e.to_string()))?;
        let next_order = match max_order { Some((Some(max),)) => max + 1, _ => 1 };

        sqlx::query("INSERT OR IGNORE INTO messages (id, json, local_order) VALUES ($1, $2, $3)")
            .bind(msg.id.to_string()).bind(json).bind(next_order).execute(&self.pool).await
            .map_err(|e| AnonError::Db(e.to_string()))?;
        Ok(())
    }

    pub async fn get_saved_session(&self) -> Result<Option<(String, String)>, AnonError> {
        sqlx::query_as("SELECT username, session_id FROM local_session LIMIT 1")
            .fetch_optional(&self.pool).await.map_err(|e| AnonError::Db(e.to_string()))
    }

    pub async fn get_server_address(&self) -> Result<String, AnonError> {
        let addr: Option<(String,)> = sqlx::query_as("SELECT address FROM server_config WHERE id = 'current' LIMIT 1")
            .fetch_optional(&self.pool).await.map_err(|e| AnonError::Db(e.to_string()))?;
        Ok(addr.map(|a| a.0).unwrap_or_else(|| "ws://127.0.0.1:3000/ws".to_string()))
    }

    pub async fn save_server_address(&self, address: &str) {
        let _ = sqlx::query("INSERT OR REPLACE INTO server_config (id, address) VALUES ('current', $1)")
            .bind(address).execute(&self.pool).await;
    }

    pub async fn get_or_generate_keys(&self) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), AnonError> {
        let row: Option<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = sqlx::query_as("SELECT ed_secret, ed_public, x25519_secret, x25519_public FROM local_keys LIMIT 1")
            .fetch_optional(&self.pool).await.map_err(|e| AnonError::Db(e.to_string()))?;

        match row {
            Some(keys) => Ok(keys),
            None => {
                let (es, ep) = shared::crypto::generate_ed25519_keys();
                let (xs, xp) = shared::crypto::generate_x25519_keys();
                sqlx::query("INSERT INTO local_keys (ed_secret, ed_public, x25519_secret, x25519_public) VALUES ($1, $2, $3, $4)")
                    .bind(&es).bind(&ep).bind(&xs).bind(&xp).execute(&self.pool).await
                    .map_err(|e| AnonError::Db(e.to_string()))?;
                Ok((es, ep, xs, xp))
            }
        }
    }


    pub async fn save_local_session(&self, username: &str, session_id: &str) {
        let _ = sqlx::query("INSERT OR REPLACE INTO local_session (id, username, session_id) VALUES (1, $1, $2)")
            .bind(username)
            .bind(session_id)
            .execute(&self.pool)
            .await;
    }

    pub async fn clear_local_session(&self) {
        let _ = sqlx::query("DELETE FROM local_session WHERE id = 1")
            .execute(&self.pool)
            .await;
    }

}