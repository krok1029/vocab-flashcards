use std::path::PathBuf;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tracing_appender::{non_blocking, rolling};

/// 日誌配置結構
pub struct LogConfig {
    pub log_dir: PathBuf,
    pub file_name_prefix: String,
    pub max_level: String,
    pub enable_console: bool,
    pub enable_file: bool,
    pub rotation: LogRotation,
}

/// 日誌輪轉配置
pub enum LogRotation {
    Daily,
    Hourly,
    Never,
}

impl Default for LogConfig {
    fn default() -> Self {
        // 獲取應用數據目錄
        let app_data_dir = get_app_data_dir();
        let log_dir = app_data_dir.join("logs");
        
        Self {
            log_dir,
            file_name_prefix: "vocab-flashcards".to_string(),
            max_level: "info".to_string(),
            enable_console: true,
            enable_file: true,
            rotation: LogRotation::Daily,
        }
    }
}

/// 初始化日誌系統
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let config = LogConfig::default();
    init_logging_with_config(config)
}

/// 使用自定義配置初始化日誌系統
pub fn init_logging_with_config(config: LogConfig) -> Result<(), Box<dyn std::error::Error>> {
    // 確保日誌目錄存在
    std::fs::create_dir_all(&config.log_dir)?;
    
    // 設定環境變數（如果沒有設定的話）
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &config.max_level);
    }
    
    // 創建環境過濾器
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("vocab_flashcards={},info", config.max_level).into());
    
    let registry = tracing_subscriber::registry().with(env_filter);
    
    // 根據配置添加不同的層
    if config.enable_console && config.enable_file {
        // 同時啟用控制台和文件輸出
        let file_appender = match config.rotation {
            LogRotation::Daily => rolling::daily(&config.log_dir, &config.file_name_prefix),
            LogRotation::Hourly => rolling::hourly(&config.log_dir, &config.file_name_prefix),
            LogRotation::Never => rolling::never(&config.log_dir, format!("{}.log", config.file_name_prefix)),
        };
        
        let (non_blocking_appender, _guard) = non_blocking(file_appender);
        
        let console_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false);
            
        let file_layer = fmt::layer()
            .with_writer(non_blocking_appender)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(false); // 文件中不使用顏色代碼
        
        registry
            .with(console_layer)
            .with(file_layer)
            .init();
            
        // 防止 guard 被丟棄
        std::mem::forget(_guard);
        
    } else if config.enable_console {
        // 只啟用控制台輸出
        let console_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false);
            
        registry.with(console_layer).init();
        
    } else if config.enable_file {
        // 只啟用文件輸出
        let file_appender = match config.rotation {
            LogRotation::Daily => rolling::daily(&config.log_dir, &config.file_name_prefix),
            LogRotation::Hourly => rolling::hourly(&config.log_dir, &config.file_name_prefix),
            LogRotation::Never => rolling::never(&config.log_dir, format!("{}.log", config.file_name_prefix)),
        };
        
        let (non_blocking_appender, _guard) = non_blocking(file_appender);
        
        let file_layer = fmt::layer()
            .with_writer(non_blocking_appender)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(false);
            
        registry.with(file_layer).init();
        
        // 防止 guard 被丟棄
        std::mem::forget(_guard);
    } else {
        // 都不啟用，使用默認配置
        registry.init();
    }
    
    tracing::info!(
        log_dir = %config.log_dir.display(),
        file_prefix = %config.file_name_prefix,
        max_level = %config.max_level,
        console_enabled = config.enable_console,
        file_enabled = config.enable_file,
        "Logger initialized successfully"
    );
    
    Ok(())
}

/// 獲取應用數據目錄
fn get_app_data_dir() -> PathBuf {
    // 嘗試使用 Tauri 的應用數據目錄
    if let Ok(app_data_dir) = std::env::var("APPDATA") {
        // Windows
        PathBuf::from(app_data_dir).join("vocab-flashcards")
    } else if let Ok(home) = std::env::var("HOME") {
        // macOS/Linux
        if cfg!(target_os = "macos") {
            PathBuf::from(home).join("Library/Application Support/vocab-flashcards")
        } else {
            PathBuf::from(home).join(".local/share/vocab-flashcards")
        }
    } else {
        // 後備選項：當前目錄
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

/// 獲取當前日誌文件路徑
pub fn get_current_log_file() -> PathBuf {
    let config = LogConfig::default();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    config.log_dir.join(format!("{}.{}.log", config.file_name_prefix, today))
}

/// 獲取日誌目錄路徑
pub fn get_log_directory() -> PathBuf {
    LogConfig::default().log_dir
}

/// 清理舊日誌文件（保留最近 N 天）
pub fn cleanup_old_logs(keep_days: u64) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = get_log_directory();
    let cutoff_time = std::time::SystemTime::now() - std::time::Duration::from_secs(keep_days * 24 * 60 * 60);
    
    if log_dir.exists() {
        for entry in std::fs::read_dir(&log_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "log") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if modified < cutoff_time {
                            if let Err(e) = std::fs::remove_file(&path) {
                                tracing::warn!("Failed to remove old log file {:?}: {}", path, e);
                            } else {
                                tracing::info!("Removed old log file: {:?}", path);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_app_data_dir() {
        let dir = get_app_data_dir();
        assert!(dir.is_absolute() || dir.starts_with("."));
    }
    
    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.file_name_prefix, "vocab-flashcards");
        assert_eq!(config.max_level, "info");
        assert!(config.enable_console);
        assert!(config.enable_file);
    }
}
