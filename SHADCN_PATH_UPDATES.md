# Shadcn-svelte 路徑更新總結

## 更新的檔案

### 1. components.json
- `"components": "$lib/components"` → `"components": "$lib/presentation/components"`
- `"utils": "$lib/utils"` → `"utils": "$lib/presentation/utils"`
- `"ui": "$lib/components/ui"` → `"ui": "$lib/presentation/components/ui"`

### 2. UI 元件檔案
以下檔案中的 utils 匯入路徑已更新：

#### Button 元件
- `src/lib/presentation/components/ui/button/button.svelte`
  - `$lib/utils.js` → `$lib/presentation/utils`

#### Input 元件
- `src/lib/presentation/components/ui/input/input.svelte`
  - `$lib/utils.js` → `$lib/presentation/utils`

#### Alert 元件
- `src/lib/presentation/components/ui/alert/alert.svelte`
- `src/lib/presentation/components/ui/alert/alert-description.svelte`
- `src/lib/presentation/components/ui/alert/alert-title.svelte`
  - 所有檔案：`$lib/utils.js` → `$lib/presentation/utils`

### 3. 頁面檔案
#### 主頁面 (+page.svelte)
- `$lib/components/ui/button/button.svelte` → `$lib/presentation/components/ui/button/button.svelte`
- `$lib/domain/dictionary` → `$lib/domain/models/dictionary`
- `$lib/usecases/searchDictionary` → `$lib/application/services/searchDictionary`
- `$lib/components/ui/alert/index.js` → `$lib/presentation/components/ui/alert/index.js`

#### 匯入頁面 (import/+page.svelte)
- `$lib/components/ui/input/index.js` → `$lib/presentation/components/ui/input/index.js`

#### 佈局檔案 (+layout.svelte)
- `$lib/components/ui/sonner/index.js` → `$lib/presentation/components/ui/sonner/index.js`

## 新的路徑結構

### Shadcn-svelte 元件路徑
- **UI 元件**: `$lib/presentation/components/ui/`
- **工具函數**: `$lib/presentation/utils/`
- **一般元件**: `$lib/presentation/components/`

### DDD 架構路徑
- **領域模型**: `$lib/domain/models/`
- **領域類型**: `$lib/domain/types/`
- **應用服務**: `$lib/application/services/`
- **應用 Stores**: `$lib/application/stores/`
- **基礎設施 API**: `$lib/infrastructure/api/`

## 注意事項

1. 所有 shadcn-svelte 元件現在都位於 `$lib/presentation/components/ui/` 下
2. 工具函數 (如 `cn`) 現在位於 `$lib/presentation/utils/` 下
3. 新的 `components.json` 配置確保未來安裝的 shadcn 元件會自動使用正確的路徑
4. 所有現有的匯入路徑都已更新以符合新的 DDD 架構

## 驗證

執行以下命令來驗證更新是否成功：
```bash
npm run dev
# 或
yarn dev
```

如果沒有編譯錯誤，表示所有路徑都已正確更新。
