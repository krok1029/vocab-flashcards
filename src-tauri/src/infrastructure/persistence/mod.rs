pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use log::{info, warn};
use std::env;
use std::path::PathBuf;

/// 應用程式名稱，用於建立資料目錄
const APP_NAME: &str = "vocab-flashcards";

/// 資料庫檔案名稱
const DB_FILE_NAME: &str = "word_cards.db";

/// 建立 word_cards 表格的 SQL 語句
const CREATE_WORD_CARDS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS word_cards (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  word TEXT NOT NULL UNIQUE,
  pos TEXT,
  definition TEXT,
  pronunciation TEXT,
  verbs TEXT,
  familiarity INTEGER DEFAULT 0,
  seen_count INTEGER DEFAULT 1,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
"#;

/// 資料庫連接錯誤類型
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed(String),
    InitializationFailed(String),
    DirectoryCreationFailed(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionFailed(msg) => write!(f, "Database connection failed: {}", msg),
            DatabaseError::InitializationFailed(msg) => write!(f, "Database initialization failed: {}", msg),
            DatabaseError::DirectoryCreationFailed(msg) => write!(f, "Directory creation failed: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

/// 建立資料庫連接並初始化表格
pub fn establish_connection() -> SqliteConnection {
    match try_establish_connection() {
        Ok(connection) => connection,
        Err(e) => {
            panic!("Fatal database error: {}", e);
        }
    }
}

/// 嘗試建立資料庫連接（可處理錯誤的版本）
fn try_establish_connection() -> Result<SqliteConnection, DatabaseError> {
    let database_url = get_database_url();
    
    // 確保資料庫目錄存在
    ensure_database_directory(&database_url)?;
    
    info!("連接資料庫: {}", database_url);
    
    // 建立連接
    let mut connection = SqliteConnection::establish(&database_url)
        .map_err(|e| DatabaseError::ConnectionFailed(format!("{}: {}", database_url, e)))?;
    
    // 初始化資料庫表格
    initialize_database(&mut connection)?;
    
    info!("資料庫連接建立成功");
    Ok(connection)
}

/// 確保資料庫目錄存在
fn ensure_database_directory(database_url: &str) -> Result<(), DatabaseError> {
    if let Some(parent) = std::path::Path::new(database_url).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| DatabaseError::DirectoryCreationFailed(format!("{}: {}", parent.display(), e)))?;
    }
    Ok(())
}

/// 取得資料庫 URL
fn get_database_url() -> String {
    // 優先使用環境變數
    if let Ok(url) = env::var("DATABASE_URL") {
        return url;
    }
    
    // 根據編譯模式決定路徑
    if cfg!(debug_assertions) {
        get_development_database_path()
    } else {
        get_production_database_path()
    }
}

/// 取得開發模式的資料庫路徑
fn get_development_database_path() -> String {
    let project_root = env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| ".".to_string());
    format!("{}/db/{}", project_root, DB_FILE_NAME)
}

/// 取得生產模式的資料庫路徑
fn get_production_database_path() -> String {
    get_app_data_directory()
        .map(|mut path| {
            path.push(DB_FILE_NAME);
            path.to_string_lossy().to_string()
        })
        .unwrap_or_else(|| {
            warn!("無法確定應用程式資料目錄，使用當前目錄");
            format!("./{}", DB_FILE_NAME)
        })
}

/// 取得應用程式資料目錄
fn get_app_data_directory() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        env::var_os("HOME").map(|home| {
            let mut path = PathBuf::from(home);
            path.push("Library");
            path.push("Application Support");
            path.push(APP_NAME);
            path
        })
    }
    
    #[cfg(target_os = "windows")]
    {
        env::var_os("APPDATA").map(|appdata| {
            let mut path = PathBuf::from(appdata);
            path.push(APP_NAME);
            path
        })
    }
    
    #[cfg(target_os = "linux")]
    {
        env::var_os("HOME").map(|home| {
            let mut path = PathBuf::from(home);
            path.push(".local");
            path.push("share");
            path.push(APP_NAME);
            path
        })
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

/// 初始化資料庫表格
fn initialize_database(connection: &mut SqliteConnection) -> Result<(), DatabaseError> {
    info!("初始化資料庫表格...");
    
    diesel::sql_query(CREATE_WORD_CARDS_TABLE)
        .execute(connection)
        .map_err(|e| DatabaseError::InitializationFailed(e.to_string()))?;
    
    info!("資料庫表格初始化完成");
    Ok(())
}

/// 測試資料庫連接是否正常
pub fn test_connection() -> Result<String, String> {
    match try_establish_connection() {
        Ok(_) => Ok("資料庫連接測試成功".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_data_directory() {
        let dir = get_app_data_directory();
        // 在測試環境中，至少應該能取得某個目錄
        // 具體路徑會根據作業系統而不同
        println!("App data directory: {:?}", dir);
    }
    
    #[test]
    fn test_database_url_generation() {
        let url = get_database_url();
        assert!(!url.is_empty());
        assert!(url.ends_with(DB_FILE_NAME));
        println!("Database URL: {}", url);
    }
}
