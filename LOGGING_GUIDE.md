# 📝 日誌系統使用指南

本專案已整合完整的日誌系統，支援控制台輸出和文件記錄，具備日誌輪轉和自動清理功能。

## 🔧 系統配置

### 日誌輸出位置

- **控制台輸出**: 開發模式下直接顯示在終端機
- **文件輸出**: 自動保存到系統應用數據目錄
  - **macOS**: `~/Library/Application Support/vocab-flashcards/logs/`
  - **Windows**: `%APPDATA%\vocab-flashcards\logs\`
  - **Linux**: `~/.local/share/vocab-flashcards/logs/`

### 日誌文件命名規則

- 格式: `vocab-flashcards.YYYY-MM-DD.log`
- 範例: `vocab-flashcards.2025-08-06.log`
- 每日自動輪轉，創建新的日誌文件

### 日誌級別

| 級別 | 說明 | 使用場景 |
|------|------|----------|
| `error` | 錯誤訊息 | 系統錯誤、異常情況 |
| `warn` | 警告訊息 | 潛在問題、非致命錯誤 |
| `info` | 一般資訊 | 正常操作、狀態變更 |
| `debug` | 除錯訊息 | 開發除錯、詳細追蹤 |

## 🚀 使用方法

### 1. 在 Rust 代碼中使用

```rust
use log::{info, warn, error, debug};
use crate::infrastructure::logger::Logger;

// 基本日誌記錄
info!("應用程式啟動");
warn!("配置文件不存在，使用預設值");
error!("資料庫連接失敗: {}", error_msg);
debug!("處理請求: {}", request_data);

// 使用 Logger 工具類
Logger::info("操作完成");
Logger::error_with_context("save_word_card", "儲存失敗");
Logger::log_api_call("/api/words", "POST");
Logger::log_db_operation("INSERT", "word_cards");
Logger::log_command_execution("save_word_card", "word: hello");
```

### 2. 配置日誌級別

編輯 `src-tauri/.env` 文件：

```env
# 設定全域日誌級別
RUST_LOG=info

# 針對特定模組設定不同級別
RUST_LOG=vocab_flashcards=debug,diesel=warn,info

# 只顯示錯誤和警告
RUST_LOG=warn

# 顯示所有日誌（包括除錯）
RUST_LOG=debug
```

### 3. 前端測試頁面

訪問 `http://localhost:1420/logger-test` 來：
- 測試不同級別的日誌記錄
- 查看日誌文件資訊
- 打開日誌目錄
- 管理日誌文件

## 📊 日誌格式

### 控制台輸出格式
```
2025-08-06T05:24:19.336Z INFO vocab_flashcards: 這是一個資訊日誌
```

### 文件輸出格式
```
2025-08-06T05:24:19.336Z INFO ThreadId(01) vocab_flashcards: src/lib.rs:25: 這是一個資訊日誌
```

包含的資訊：
- 時間戳記（UTC）
- 日誌級別
- 執行緒 ID（僅文件）
- 模組名稱
- 原始碼位置（僅文件）
- 日誌訊息

## 🔄 自動管理功能

### 日誌輪轉
- **每日輪轉**: 每天自動創建新的日誌文件
- **文件命名**: 包含日期的檔名，便於管理
- **無縫切換**: 午夜時自動切換到新文件

### 自動清理
- **保留期限**: 預設保留最近 7 天的日誌
- **啟動清理**: 應用程式啟動時自動清理舊文件
- **安全刪除**: 只刪除過期的 `.log` 文件

## 🛠️ 進階配置

### 自定義日誌配置

```rust
use crate::infrastructure::logging::{LogConfig, LogRotation, init_logging_with_config};

let config = LogConfig {
    log_dir: PathBuf::from("/custom/log/path"),
    file_name_prefix: "my-app".to_string(),
    max_level: "debug".to_string(),
    enable_console: true,
    enable_file: true,
    rotation: LogRotation::Hourly, // 每小時輪轉
};

init_logging_with_config(config)?;
```

### 可用的輪轉選項

- `LogRotation::Daily` - 每日輪轉（預設）
- `LogRotation::Hourly` - 每小時輪轉
- `LogRotation::Never` - 不輪轉，單一文件

## 📋 最佳實踐

### 1. 日誌級別選擇
- **生產環境**: 使用 `info` 或 `warn`
- **開發環境**: 使用 `debug`
- **問題排查**: 臨時調整為 `debug`

### 2. 日誌內容建議
```rust
// ✅ 好的做法
info!("用戶 {} 成功登入", user_id);
error!("資料庫操作失敗: table={}, operation={}, error={}", table, op, err);

// ❌ 避免的做法
info!("something happened"); // 資訊不足
error!("{}", err); // 缺乏上下文
```

### 3. 敏感資訊處理
```rust
// ✅ 安全的做法
info!("用戶認證成功: user_id={}", user_id);

// ❌ 不安全的做法
debug!("用戶密碼: {}", password); // 絕對不要記錄密碼
```

## 🔍 故障排除

### 日誌文件未創建
1. 檢查應用數據目錄權限
2. 確認 `LOG_ENABLE_FILE=true` 設定
3. 查看控制台是否有初始化錯誤

### 日誌級別不生效
1. 檢查 `.env` 文件中的 `RUST_LOG` 設定
2. 重新啟動應用程式
3. 確認環境變數正確載入

### 日誌文件過大
1. 檢查日誌輪轉是否正常工作
2. 調整日誌級別，減少 `debug` 訊息
3. 手動清理舊日誌文件

## 📚 相關命令

### Tauri 命令
- `test_logger` - 測試日誌功能
- `test_error_logging` - 測試錯誤日誌
- `get_log_info` - 獲取日誌文件資訊
- `open_log_directory` - 打開日誌目錄

### 測試命令
```bash
# 運行日誌測試
cd src-tauri
cargo run --bin test_logger

# 查看日誌文件
cat ~/Library/Application\ Support/vocab-flashcards/logs/vocab-flashcards.$(date +%Y-%m-%d).log
```

## 🎯 總結

本日誌系統提供了：
- ✅ 雙重輸出（控制台 + 文件）
- ✅ 自動日誌輪轉
- ✅ 智能清理機制
- ✅ 靈活的級別控制
- ✅ 結構化日誌支援
- ✅ 前端管理介面
- ✅ 跨平台相容性

透過這個系統，你可以有效地監控和除錯應用程式的運行狀態！
