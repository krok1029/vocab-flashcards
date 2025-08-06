use log::{info, warn, error, debug};

/// Logger 工具結構
pub struct Logger;

impl Logger {
    /// 記錄資訊級別日誌
    pub fn info(message: &str) {
        info!("{}", message);
    }

    /// 記錄警告級別日誌
    pub fn warn(message: &str) {
        warn!("{}", message);
    }

    /// 記錄錯誤級別日誌
    pub fn error(message: &str) {
        error!("{}", message);
    }

    /// 記錄除錯級別日誌
    pub fn debug(message: &str) {
        debug!("{}", message);
    }

    /// 記錄帶有上下文的資訊日誌
    pub fn info_with_context(context: &str, message: &str) {
        info!("[{}] {}", context, message);
    }

    /// 記錄帶有上下文的錯誤日誌
    pub fn error_with_context(context: &str, message: &str) {
        error!("[{}] {}", context, message);
    }

    /// 記錄 API 呼叫
    pub fn log_api_call(endpoint: &str, method: &str) {
        info!("API Call: {} {}", method, endpoint);
    }

    /// 記錄資料庫操作
    pub fn log_db_operation(operation: &str, table: &str) {
        debug!("DB Operation: {} on table '{}'", operation, table);
    }

    /// 記錄 Tauri 命令執行
    pub fn log_command_execution(command: &str, params: &str) {
        info!("Tauri Command: {} with params: {}", command, params);
    }
}

/// 便利的巨集定義
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    };
}
