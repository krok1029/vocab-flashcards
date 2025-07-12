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

// 現有的範例指令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 真正啟動 app 的地方
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                greet, // ✅ 這裡要有
                presentation::commands::word_cards::save_word_card,
                presentation::commands::word_cards::get_word_card_by_word,
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
