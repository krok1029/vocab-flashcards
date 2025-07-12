# DDD 架構說明

本專案已重新組織為 Domain-Driven Design (DDD) 架構，分為前端 (SvelteKit) 和後端 (Rust/Tauri) 兩部分。

## 後端架構 (src-tauri/src)

### 📁 Domain Layer (領域層)
- `domain/entities/` - 實體 (如 WordCard)
- `domain/value_objects/` - 值物件
- `domain/aggregates/` - 聚合根
- `domain/repositories/` - 倉儲介面
- `domain/services/` - 領域服務

### 📁 Application Layer (應用層)
- `application/commands/` - 命令處理器 (CQRS)
- `application/queries/` - 查詢處理器 (CQRS)
- `application/dto/` - 資料傳輸物件
- `application/services/` - 應用服務

### 📁 Infrastructure Layer (基礎設施層)
- `infrastructure/persistence/` - 資料持久化 (包含 schema.rs)
- `infrastructure/repositories/` - 倉儲實作
- `infrastructure/external/` - 外部服務整合

### 📁 Presentation Layer (表現層)
- `presentation/commands/` - Tauri 命令處理

## 前端架構 (src/lib)

### 📁 Domain Layer (領域層)
- `domain/models/` - 領域模型
- `domain/types/` - TypeScript 類型定義

### 📁 Application Layer (應用層)
- `application/services/` - 應用服務
- `application/stores/` - Svelte 狀態管理
- `application/commands/` - 命令模式

### 📁 Infrastructure Layer (基礎設施層)
- `infrastructure/api/` - API 呼叫
- `infrastructure/storage/` - 本地儲存
- `infrastructure/tauri/` - Tauri 整合

### 📁 Presentation Layer (表現層)
- `presentation/components/` - UI 元件
- `presentation/layouts/` - 版面配置
- `presentation/utils/` - 工具函數

## DDD 核心原則

1. **依賴反轉**: Domain 層不依賴任何外部層
2. **清晰分層**: 每層都有明確的職責
3. **聚合設計**: 相關實體組織成聚合
4. **CQRS**: 分離命令和查詢操作
5. **領域驅動**: 業務邏輯集中在 Domain 層

## 使用指南

### 新增功能時的步驟：
1. 在 Domain 層定義實體和業務規則
2. 在 Application 層實作用例
3. 在 Infrastructure 層實作技術細節
4. 在 Presentation 層暴露介面

### 檔案命名規範：
- 使用清晰的命名反映業務概念
- 每個模組都有對應的 mod.rs (Rust) 或 index.ts (TypeScript)
- 保持一致的匯出模式

這樣的架構讓程式碼更容易維護、測試和擴展，同時保持了業務邏輯的純淨性。
