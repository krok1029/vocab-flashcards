// src/lib.rs

// 現有的範例指令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 匯入模組（新增的）
mod schema;
mod db;

pub mod models {
    pub mod word_cards;
}

pub mod commands {
    pub mod word_cards;
}

// 真正啟動 app 的地方
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                greet, // ✅ 這裡要有
                commands::word_cards::save_word_card,
                commands::word_cards::get_word_card_by_word,
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
