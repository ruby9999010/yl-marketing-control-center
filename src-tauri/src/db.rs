use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions, SqliteJournalMode}, Pool, Sqlite, FromRow};
use anyhow::{Context, Result};
use std::str::FromStr;
use std::time::Duration;
use directories::ProjectDirs;

#[derive(Debug, FromRow, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub id: i64,
    pub client_id: String,
    pub display_name: Option<String>,
    pub registered_at: String,
    pub last_seen: Option<String>,
    pub is_active: i64,
}

#[derive(Debug, FromRow, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMemo {
    pub id: i64,
    pub client_id: String,
    pub memo: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub id: i64,
    pub client_id: String,
    pub level: String,
    pub message: String,
    pub timestamp: String,
    pub metadata: Option<String>,
}

#[derive(Debug, FromRow, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientStatus {
    pub client_id: String,
    pub success_count: i64,
    pub failure_count: i64,
    pub next_execution_in: i64,
    pub current_status: String,
    pub last_activity: Option<String>,
    pub updated_at: String,
}

pub struct Db {
    pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "svelion", "yl-marketing-control-center")
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
            
        let data_dir = proj_dirs.data_dir();
        tokio::fs::create_dir_all(data_dir).await
            .context("Failed to create data directory")?;
            
        let db_path = data_dir.join("control_center.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());
        
        println!("📂 Database path: {:?}", db_path);

        let options = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_secs(30));

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .context("Failed to connect to SQLite database")?;

        // 테이블 생성
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS clients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                client_id TEXT NOT NULL UNIQUE,
                display_name TEXT,
                registered_at TEXT DEFAULT (datetime('now', 'localtime')),
                last_seen TEXT,
                is_active INTEGER DEFAULT 1
            );
            "#
        )
        .execute(&pool)
        .await
        .context("Failed to create clients table")?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS client_memos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                client_id TEXT NOT NULL UNIQUE,
                memo TEXT NOT NULL DEFAULT '',
                updated_at TEXT DEFAULT (datetime('now', 'localtime')),
                FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
            );
            "#
        )
        .execute(&pool)
        .await
        .context("Failed to create client_memos table")?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS log_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                client_id TEXT NOT NULL,
                level TEXT NOT NULL,
                message TEXT NOT NULL,
                timestamp TEXT DEFAULT (datetime('now', 'localtime')),
                metadata TEXT,
                FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
            );
            "#
        )
        .execute(&pool)
        .await
        .context("Failed to create log_history table")?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS client_status (
                client_id TEXT PRIMARY KEY,
                success_count INTEGER DEFAULT 0,
                failure_count INTEGER DEFAULT 0,
                next_execution_in INTEGER DEFAULT 0,
                current_status TEXT DEFAULT 'unknown',
                last_activity TEXT,
                updated_at TEXT DEFAULT (datetime('now', 'localtime')),
                FOREIGN KEY (client_id) REFERENCES clients(client_id) ON DELETE CASCADE
            );
            "#
        )
        .execute(&pool)
        .await
        .context("Failed to create client_status table")?;

        // 인덱스 생성
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_log_history_client_id ON log_history(client_id)")
            .execute(&pool)
            .await
            .ok();

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_log_history_timestamp ON log_history(timestamp DESC)")
            .execute(&pool)
            .await
            .ok();

        Ok(Self { pool })
    }

    // 클라이언트 관리
    pub async fn register_client(&self, client_id: &str, display_name: Option<&str>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO clients (client_id, display_name, is_active)
            VALUES (?, ?, 1)
            ON CONFLICT(client_id) DO UPDATE SET
                last_seen = datetime('now', 'localtime'),
                is_active = 1
            "#
        )
        .bind(client_id)
        .bind(display_name)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_client_last_seen(&self, client_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE clients SET last_seen = datetime('now', 'localtime') WHERE client_id = ?"
        )
        .bind(client_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_clients(&self) -> Result<Vec<Client>> {
        let clients = sqlx::query_as::<_, Client>(
            "SELECT id, client_id, display_name, registered_at, last_seen, is_active FROM clients ORDER BY registered_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }

    pub async fn get_client(&self, client_id: &str) -> Result<Option<Client>> {
        let client = sqlx::query_as::<_, Client>(
            "SELECT id, client_id, display_name, registered_at, last_seen, is_active FROM clients WHERE client_id = ?"
        )
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(client)
    }

    pub async fn deactivate_client(&self, client_id: &str) -> Result<()> {
        sqlx::query("UPDATE clients SET is_active = 0 WHERE client_id = ?")
            .bind(client_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // 메모 관리
    pub async fn save_memo(&self, client_id: &str, memo: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO client_memos (client_id, memo, updated_at)
            VALUES (?, ?, datetime('now', 'localtime'))
            ON CONFLICT(client_id) DO UPDATE SET
                memo = excluded.memo,
                updated_at = datetime('now', 'localtime')
            "#
        )
        .bind(client_id)
        .bind(memo)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_memo(&self, client_id: &str) -> Result<Option<ClientMemo>> {
        let memo = sqlx::query_as::<_, ClientMemo>(
            "SELECT id, client_id, memo, updated_at FROM client_memos WHERE client_id = ?"
        )
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(memo)
    }

    pub async fn list_memos(&self) -> Result<Vec<ClientMemo>> {
        let memos = sqlx::query_as::<_, ClientMemo>(
            "SELECT id, client_id, memo, updated_at FROM client_memos ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(memos)
    }

    // 로그 관리
    pub async fn add_log(
        &self,
        client_id: &str,
        level: &str,
        message: &str,
        metadata: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO log_history (client_id, level, message, metadata) VALUES (?, ?, ?, ?)"
        )
        .bind(client_id)
        .bind(level)
        .bind(message)
        .bind(metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_logs(&self, client_id: Option<&str>, limit: i64) -> Result<Vec<LogEntry>> {
        let logs = if let Some(cid) = client_id {
            sqlx::query_as::<_, LogEntry>(
                "SELECT id, client_id, level, message, timestamp, metadata FROM log_history WHERE client_id = ? ORDER BY timestamp DESC LIMIT ?"
            )
            .bind(cid)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, LogEntry>(
                "SELECT id, client_id, level, message, timestamp, metadata FROM log_history ORDER BY timestamp DESC LIMIT ?"
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(logs)
    }

    pub async fn clear_old_logs(&self, days: i64) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM log_history WHERE timestamp < datetime('now', '-' || ? || ' days')"
        )
        .bind(days)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    // 상태 관리
    pub async fn update_status(
        &self,
        client_id: &str,
        success_count: i64,
        failure_count: i64,
        next_execution_in: i64,
        current_status: &str,
        last_activity: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO client_status (client_id, success_count, failure_count, next_execution_in, current_status, last_activity, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, datetime('now', 'localtime'))
            ON CONFLICT(client_id) DO UPDATE SET
                success_count = excluded.success_count,
                failure_count = excluded.failure_count,
                next_execution_in = excluded.next_execution_in,
                current_status = excluded.current_status,
                last_activity = excluded.last_activity,
                updated_at = datetime('now', 'localtime')
            "#
        )
        .bind(client_id)
        .bind(success_count)
        .bind(failure_count)
        .bind(next_execution_in)
        .bind(current_status)
        .bind(last_activity)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_status(&self, client_id: &str) -> Result<Option<ClientStatus>> {
        let status = sqlx::query_as::<_, ClientStatus>(
            "SELECT client_id, success_count, failure_count, next_execution_in, current_status, last_activity, updated_at FROM client_status WHERE client_id = ?"
        )
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(status)
    }

    pub async fn list_statuses(&self) -> Result<Vec<ClientStatus>> {
        let statuses = sqlx::query_as::<_, ClientStatus>(
            "SELECT client_id, success_count, failure_count, next_execution_in, current_status, last_activity, updated_at FROM client_status ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(statuses)
    }
}
