# Rust DDD 架構修復總結

## 修復的錯誤

### 1. 模組路徑更新

#### `src-tauri/src/domain/entities/word_cards.rs`
**修復前的問題：**
- 使用舊路徑 `crate::schema::word_cards`

**修復後：**
```rust
// 更新匯入路徑
use crate::infrastructure::persistence::schema::word_cards;
```

#### `src-tauri/src/presentation/commands/word_cards.rs`
**修復前的問題：**
- 使用舊路徑 `crate::db::establish_connection`
- 使用舊路徑 `crate::models::word_cards::{NewWordCard, WordCard}`
- 使用舊路徑 `crate::schema::word_cards::dsl::*`

**修復後：**
```rust
// 更新所有匯入路徑
use crate::infrastructure::persistence::establish_connection;
use crate::domain::entities::word_cards::{NewWordCard, WordCard};
use crate::infrastructure::persistence::schema::word_cards::dsl::*;
```

### 2. 基礎設施層完善

#### `src-tauri/src/infrastructure/persistence/mod.rs`
**新增內容：**
```rust
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "db/word_cards.db".to_string());
    
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```

### 3. Diesel 配置更新

#### `src-tauri/diesel.toml`
**修復前的問題：**
- schema 檔案路徑指向舊位置 `src/schema.rs`

**修復後：**
```toml
[print_schema]
file = "src/infrastructure/persistence/schema.rs"
```

**驗證：**
- `diesel print-schema` 命令正常工作
- 未來的 migration 會正確更新 schema 檔案

## 新的模組結構

### Domain Layer (領域層)
```
src-tauri/src/domain/
├── entities/
│   └── word_cards.rs     # WordCard, NewWordCard 實體
├── value_objects/
├── aggregates/
├── repositories/
└── services/
```

### Infrastructure Layer (基礎設施層)
```
src-tauri/src/infrastructure/
├── persistence/
│   ├── mod.rs           # establish_connection 函數
│   └── schema.rs        # Diesel schema 定義
├── repositories/
└── external/
```

### Presentation Layer (表現層)
```
src-tauri/src/presentation/
└── commands/
    └── word_cards.rs    # Tauri 命令處理器
```

## DDD 原則實現

1. **依賴反轉**: 
   - Domain 層的實體不依賴基礎設施
   - Presentation 層依賴 Domain 和 Infrastructure

2. **清晰分層**:
   - 實體定義在 Domain 層
   - 資料庫連接在 Infrastructure 層
   - Tauri 命令在 Presentation 層

3. **模組化**:
   - 每層都有清晰的職責
   - 匯入路徑反映架構層次

## 編譯驗證

- ✅ `cargo check` 成功，沒有編譯錯誤
- ✅ `diesel print-schema` 正常工作
- ✅ 所有模組路徑正確

## Diesel 工作流程

現在當你需要：

1. **創建新的 migration**:
   ```bash
   diesel migration generate create_new_table
   ```

2. **運行 migration**:
   ```bash
   diesel migration run
   ```

3. **更新 schema**:
   - 會自動更新 `src/infrastructure/persistence/schema.rs`
   - 符合 DDD 架構的檔案組織

## 下一步建議

1. 考慮將資料庫操作抽象為 Repository 模式
2. 在 Application 層添加業務邏輯服務
3. 實現 CQRS 模式分離命令和查詢
