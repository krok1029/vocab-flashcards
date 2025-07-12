# 主頁面重構說明

## 重構目標

將原本的 `src/routes/+page.svelte` 從單一檔案重構為符合 DDD 架構的模組化結構，提升程式碼的可維護性、可測試性和可重用性。

## 主要改進

### 1. 架構分層 (DDD Architecture)

#### Domain Layer (領域層)
- **新增**: `src/lib/domain/types/wordCard.ts` - 完整的 WordCard 類型定義
- **改進**: 更完整的類型定義，包含所有必要欄位

#### Application Layer (應用層)
- **新增**: `src/lib/application/stores/dictionaryStore.ts` - 字典查詢狀態管理
- **新增**: `src/lib/application/commands/searchCommand.ts` - 搜尋與儲存命令
- **新增**: `src/lib/application/services/wordCardService.ts` - 單字卡服務

#### Presentation Layer (表現層)
- **新增**: `src/lib/presentation/components/SearchInput.svelte` - 搜尋輸入元件
- **新增**: `src/lib/presentation/components/WordDisplay.svelte` - 單字顯示元件
- **新增**: `src/lib/presentation/components/LoadingSpinner.svelte` - 載入動畫元件
- **新增**: `src/lib/presentation/components/ErrorDisplay.svelte` - 錯誤顯示元件
- **新增**: `src/lib/presentation/layouts/MainLayout.svelte` - 主要版面配置
- **新增**: `src/lib/presentation/utils/pronunciationUtils.ts` - 發音資料處理工具

### 2. 元件化 (Component Separation)

#### 原本問題
- 所有邏輯都在單一檔案中
- UI 與業務邏輯混合
- 難以重用和測試

#### 改進後
- **SearchInput**: 獨立的搜尋輸入元件，處理使用者輸入與事件
- **WordDisplay**: 專門顯示單字資訊的元件，包含發音播放功能
- **LoadingSpinner**: 可重用的載入動畫元件
- **ErrorDisplay**: 統一的錯誤顯示元件，支援重試功能
- **MainLayout**: 提供一致的頁面結構與導航

### 3. 狀態管理 (State Management)

#### 原本問題
- 使用 `let` 變數管理狀態
- 狀態散佈在各處
- 難以追蹤狀態變化

#### 改進後
- **dictionaryStore**: 集中管理字典查詢相關狀態
- **Derived Stores**: 計算屬性，如 `hasEntry`、`canSaveCard`
- **Reactive Updates**: 自動響應狀態變化

### 4. 命令模式 (Command Pattern)

#### 原本問題
- 業務邏輯直接寫在元件中
- 難以測試和重用

#### 改進後
- **SearchCommand**: 處理搜尋邏輯
- **SaveWordCardCommand**: 處理儲存邏輯
- 清晰的職責分離

### 5. 服務層 (Service Layer)

#### 原本問題
- API 呼叫邏輯散佈各處
- 資料轉換邏輯重複

#### 改進後
- **WordCardService**: 統一處理單字卡相關操作
- 資料轉換邏輯集中管理
- 錯誤處理標準化

### 6. 使用者體驗改進

#### 視覺改進
- 更好的版面配置與導航
- 一致的設計語言
- 改進的載入與錯誤狀態顯示

#### 互動改進
- 更清晰的狀態回饋
- 改進的錯誤處理與重試機制
- 更好的無障礙支援

## 檔案結構對比

### 重構前
```
src/routes/+page.svelte (200+ 行，包含所有邏輯)
```

### 重構後
```
src/
├── routes/+page.svelte (簡潔的主頁面，50 行)
├── lib/
│   ├── domain/types/wordCard.ts
│   ├── application/
│   │   ├── stores/dictionaryStore.ts
│   │   ├── commands/searchCommand.ts
│   │   └── services/wordCardService.ts
│   └── presentation/
│       ├── components/
│       │   ├── SearchInput.svelte
│       │   ├── WordDisplay.svelte
│       │   ├── LoadingSpinner.svelte
│       │   └── ErrorDisplay.svelte
│       ├── layouts/MainLayout.svelte
│       └── utils/pronunciationUtils.ts
```

## 優勢

### 1. 可維護性
- 職責清晰分離
- 模組化結構
- 易於定位和修改問題

### 2. 可測試性
- 業務邏輯與 UI 分離
- 純函數和服務類別
- 易於單元測試

### 3. 可重用性
- 元件可在其他頁面重用
- 服務和工具函數可共享
- 一致的設計模式

### 4. 可擴展性
- 清晰的架構邊界
- 易於添加新功能
- 符合 SOLID 原則

### 5. 開發體驗
- 更好的 TypeScript 支援
- 清晰的匯入結構
- 一致的程式碼風格

## 後續改進建議

1. **測試覆蓋**: 為服務和命令添加單元測試
2. **錯誤邊界**: 實作更完整的錯誤處理機制
3. **效能優化**: 添加適當的快取機制
4. **無障礙支援**: 改進鍵盤導航和螢幕閱讀器支援
5. **國際化**: 準備多語言支援結構

## 遷移指南

如果需要在其他頁面應用類似的重構：

1. 識別業務邏輯並移至 Application Layer
2. 將 UI 元件拆分為獨立的 Svelte 元件
3. 使用 Stores 管理狀態
4. 實作 Command Pattern 處理使用者操作
5. 建立 Service 類別處理外部 API 呼叫
6. 使用統一的 Layout 元件
