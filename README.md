# 📘 單字卡查詢與複習工具

一款支援查詢字典、自動建立單字卡、匯入未知單字並依熟悉度進行複習的桌面應用程式。

![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)
![SvelteKit](https://img.shields.io/badge/SvelteKit-orange?logo=svelte)
![TypeScript](https://img.shields.io/badge/TypeScript-blue?logo=typescript)
![Rust](https://img.shields.io/badge/Rust-black?logo=rust)

## ✨ 主要功能

### 🔍 字典查詢
- 手動輸入單字查詢
- 顯示完整資訊：詞性、定義、例句、發音連結、動詞變化
- 支援播放發音
- 一鍵加入為單字卡

### 📥 批次匯入
- 匯入純文字檔，每行一筆單字或片語
- 自動去除空行與重複單字
- 記錄每個單字在檔案中的出現次數
- 智能過濾：僅查詢資料庫中尚未存在的單字
- 批次查詢字典 API 並建立單字卡

### 🃏 單字卡系統
- 卡片式呈現（正面/背面）
- 完整資訊展示：詞性、定義、例句、發音、動詞變化
- 熟悉度等級系統（0～3）
- 支援手動調整熟悉度

### 🔁 複習模式 (開發中)
- 根據熟悉度調整出現頻率
- 每日建議複習清單
- 答對/答錯後自動調整熟悉度

### 📊 統計管理 (規劃中)
- 單字數量統計（依熟悉度分類）
- 匯入記錄與學習進度追蹤

## 🛠️ 技術架構

### 前端技術棧
- **框架**: SvelteKit + TypeScript
- **UI**: Tailwind CSS + shadcn-svelte
- **通知**: svelte-sonner
- **狀態管理**: Svelte Stores

### 後端技術棧
- **桌面框架**: Tauri 2.0
- **語言**: Rust
- **資料庫**: SQLite
- **ORM**: Diesel

### 架構設計
- **設計模式**: Domain-Driven Design (DDD)
- **分層架構**: Domain / Application / Infrastructure / Presentation
- **API 整合**: [Eliaschen Dictionary API](https://dictionary-api.eliaschen.dev/)

## 📁 專案結構

```
vocab-flashcards/
├── src/                          # 前端 SvelteKit
│   ├── lib/
│   │   ├── domain/              # 領域層
│   │   │   ├── models/          # 領域模型
│   │   │   └── types/           # TypeScript 類型
│   │   ├── application/         # 應用層
│   │   │   ├── services/        # 應用服務
│   │   │   ├── stores/          # 狀態管理
│   │   │   └── commands/        # 命令模式
│   │   ├── infrastructure/      # 基礎設施層
│   │   │   ├── api/             # API 呼叫
│   │   │   ├── storage/         # 本地儲存
│   │   │   └── tauri/           # Tauri 整合
│   │   └── presentation/        # 表現層
│   │       ├── components/      # UI 元件
│   │       ├── layouts/         # 版面配置
│   │       └── utils/           # 工具函數
│   └── routes/                  # SvelteKit 路由
└── src-tauri/                   # 後端 Rust
    └── src/
        ├── domain/              # 領域層
        │   ├── entities/        # 實體
        │   ├── value_objects/   # 值物件
        │   ├── aggregates/      # 聚合根
        │   ├── repositories/    # 倉儲介面
        │   └── services/        # 領域服務
        ├── application/         # 應用層
        │   ├── commands/        # 命令處理器
        │   ├── queries/         # 查詢處理器
        │   ├── dto/             # 資料傳輸物件
        │   └── services/        # 應用服務
        ├── infrastructure/      # 基礎設施層
        │   ├── persistence/     # 資料持久化
        │   ├── repositories/    # 倉儲實作
        │   └── external/        # 外部服務
        └── presentation/        # 表現層
            └── commands/        # Tauri 命令
```

## 🚀 快速開始

### 環境需求
- Node.js 18+
- Rust 1.70+
- Tauri CLI

### 安裝依賴
```bash
# 安裝前端依賴
npm install

# 安裝 Tauri CLI (如果尚未安裝)
npm install -g @tauri-apps/cli
```

### 開發模式
```bash
# 啟動開發伺服器
npm run tauri dev
```

### 建置應用程式
```bash
# 建置桌面應用程式
npm run tauri build
```

### 資料庫設定
```bash
# 進入 Rust 專案目錄
cd src-tauri

# 安裝 Diesel CLI (如果尚未安裝)
cargo install diesel_cli --no-default-features --features sqlite

# 執行資料庫遷移
diesel migration run
```

## 📦 功能進度

| 模組 | 狀態 | 說明 |
|------|------|------|
| 🔍 字典查詢 | ✅ 已完成 | 支援單字查詢與發音播放 |
| 📥 匯入 TXT | ✅ 已完成 | 批次匯入與重複比對 |
| 🃏 單字卡系統 | ✅ 基礎完成 | 卡片展示與熟悉度管理 |
| 🔁 複習模式 | ⏳ 開發中 | 間隔重複演算法 |
| 📊 統計視覺化 | ⏳ 規劃中 | 學習進度追蹤 |

## 🔜 待開發功能

- [ ] 間隔重複演算法 (Spaced Repetition)
- [ ] 搜尋歷史與單字篩選
- [ ] 單字卡分類（標籤系統）
- [ ] 學習統計視覺化
- [ ] 匯出學習記錄

## 🛠️ 開發工具推薦

- **IDE**: [VS Code](https://code.visualstudio.com/)
- **擴展**:
  - [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 📄 相關文件

- [DDD 架構說明](./DDD_ARCHITECTURE.md)
- [功能詳細規格](./feature.md)
- [Shadcn 路徑更新](./SHADCN_PATH_UPDATES.md)
- [Rust 架構修復](./RUST_DDD_FIXES.md)

## 🤝 貢獻

歡迎提交 Issue 和 Pull Request！

## 📝 授權

MIT License
