use log::{info, warn};
use crate::infrastructure::logging;

// Domain layer - 領域層
pub mod domain;

// Application layer - 應用層
pub mod application;

// Infrastructure layer - 基礎設施層
pub mod infrastructure;

// Presentation layer - 表現層 (Tauri Commands)
pub mod presentation;

// Re-export commonly used items
pub use domain::entities;
pub use presentation::commands;

// 初始化 logger
fn init_logger() {
    // 使用新的日誌配置系統
    if let Err(e) = logging::init_logging() {
        eprintln!("Failed to initialize logger: {}", e);
        // 如果文件日誌初始化失敗，回退到簡單的控制台日誌
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        env_logger::init();
    }

    // 清理 7 天前的舊日誌文件
    if let Err(e) = logging::cleanup_old_logs(7) {
        warn!("Failed to cleanup old logs: {}", e);
    }

    info!("Application logger initialized");
    info!("Log directory: {}", logging::get_log_directory().display());
    info!("Current log file: {}", logging::get_current_log_file().display());
}

// 現有的範例指令
#[tauri::command]
fn greet(name: &str) -> String {
    info!("Greet command called with name: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 真正啟動 app 的地方
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化 logger
    init_logger();
    
    info!("Starting Tauri application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                greet, // ✅ 這裡要有
                presentation::commands::word_cards::save_word_card,
                presentation::commands::word_cards::get_word_card_by_word,
                presentation::commands::word_cards::get_all_word_cards,
                presentation::commands::word_cards::update_word_card_familiarity,
                presentation::commands::word_cards::delete_word_card,
                presentation::commands::word_cards::increment_word_card_seen_count,
                // 測試和調試命令
                presentation::commands::word_cards::get_all_word_cards_simple,
                presentation::commands::word_cards::test_database_connection,
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
