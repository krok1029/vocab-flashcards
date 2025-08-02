<script lang="ts">
  import { onMount } from 'svelte';
  import { Input } from '$lib/presentation/components/ui/input/index.js';
  import { Button } from '$lib/presentation/components/ui/button/index.js';
  import { Badge } from '$lib/presentation/components/ui/badge/index.js';
  import * as Alert from '$lib/presentation/components/ui/alert/index.js';
  import { MainLayout } from '$lib/presentation/layouts';
  import { ImportCommand } from '$lib/application/commands';
  import {
    importStore,
    isImporting,
    hasResults,
  } from '$lib/application/stores';
  import ImportProgress from '$lib/presentation/components/ImportProgress.svelte';
  import ImportResults from '$lib/presentation/components/ImportResults.svelte';
  import { Upload, FileText, AlertTriangle, Info } from '@lucide/svelte';

  let files = $state<FileList | undefined>(undefined);
  let fileContent = $state('');
  let lines = $state<string[]>([]);
  let dragOver = $state(false);

  // Reactive effects
  $effect(() => {
    if (files && files.length > 0) {
      processFile(files[0]);
    }
  });

  let canImport = $derived(lines.length > 0 && !$isImporting);
  let hasPreview = $derived(lines.length > 0);

  function processFile(file: File) {
    if (!file.name.endsWith('.txt')) {
      alert('請選擇 .txt 檔案');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      fileContent = e.target?.result as string;

      // 處理文字內容
      let rawLines = fileContent
        .split('\n')
        .map((l) => l.trim().toLowerCase())
        .filter((l) => l.length > 0)
        .filter((l) => /^[a-zA-Z\s-']+$/.test(l)); // 只保留英文單字

      // 去重並排序
      let uniqueWords = [...new Set(rawLines)].sort();
      lines = uniqueWords;
    };
    reader.readAsText(file);
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    dragOver = true;
  }

  function handleDragLeave(event: DragEvent) {
    event.preventDefault();
    dragOver = false;
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    dragOver = false;

    const droppedFiles = event.dataTransfer?.files;
    if (droppedFiles && droppedFiles.length > 0) {
      files = droppedFiles;
    }
  }

  async function handleImport() {
    if (lines.length === 0) return;
    await ImportCommand.execute(lines);
  }

  function handleReset() {
    files = undefined;
    fileContent = '';
    lines = [];
    importStore.reset();
  }

  // Cleanup on component destroy
  onMount(() => {
    return () => {
      if (!$hasResults) {
        importStore.reset();
      }
    };
  });
</script>

<MainLayout
  title="批次匯入 - 單字卡工具"
  description="匯入 TXT 檔案，批次建立單字卡"
>
  <div class="space-y-6 max-w-4xl mx-auto">
    <!-- Page Header -->
    <header class="text-center space-y-2">
      <h1 class="text-3xl font-bold text-gray-800">📥 批次匯入</h1>
      <p class="text-gray-600">上傳 TXT 檔案，自動查詢並建立單字卡</p>
    </header>

    <!-- Instructions -->
    <Alert.Root class="border-blue-200 bg-blue-50">
      <Info class="h-4 w-4" />
      <Alert.Title class="text-blue-800">使用說明</Alert.Title>
      <Alert.Description class="text-blue-700 space-y-1">
        <p>• 支援 .txt 檔案格式，每行一個單字或片語</p>
        <p>• 系統會自動去除重複單字並過濾非英文內容</p>
        <p>• 已存在的單字會被跳過，避免重複匯入</p>
        <p>• 匯入過程中會自動查詢字典 API 並儲存單字卡</p>
      </Alert.Description>
    </Alert.Root>

    <!-- File Upload Section -->
    <section class="space-y-4">
      <h2 class="text-xl font-semibold text-gray-800">📁 選擇檔案</h2>

      <!-- Drag & Drop Area -->
      <div
        class="border-2 border-dashed rounded-lg p-8 text-center transition-colors {dragOver
          ? 'border-blue-400 bg-blue-50'
          : 'border-gray-300 hover:border-gray-400'}"
        ondragover={handleDragOver}
        ondragleave={handleDragLeave}
        ondrop={handleDrop}
        role="button"
        tabindex="0"
      >
        <div class="space-y-4">
          <Upload class="w-12 h-12 mx-auto text-gray-400" />
          <div class="space-y-2">
            <p class="text-lg font-medium text-gray-700">
              拖放檔案到此處或點擊選擇
            </p>
            <p class="text-sm text-gray-500">支援 .txt 檔案，每行一個單字</p>
          </div>
          <Input
            type="file"
            accept=".txt"
            bind:files
            class="max-w-xs mx-auto"
          />
        </div>
      </div>

      <!-- File Info -->
      {#if files && files.length > 0}
        <div class="flex items-center gap-2 p-3 bg-gray-50 rounded-lg">
          <FileText class="w-5 h-5 text-gray-600" />
          <span class="font-medium text-gray-800">{files[0].name}</span>
          <Badge variant="secondary">
            {(files[0].size / 1024).toFixed(1)} KB
          </Badge>
        </div>
      {/if}
    </section>

    <!-- Preview Section -->
    {#if hasPreview}
      <section class="space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold text-gray-800">👀 預覽內容</h2>
          <div class="flex items-center gap-2">
            <Badge variant="outline" class="text-sm">
              共 {lines.length} 個單字
            </Badge>
            <Button variant="outline" size="sm" onclick={handleReset}>
              重新選擇
            </Button>
          </div>
        </div>

        <!-- Word List Preview -->
        <div class="bg-gray-50 border rounded-lg max-h-[300px] overflow-y-auto">
          <div class="p-4 border-b bg-gray-100">
            <p class="text-sm font-medium text-gray-700">
              單字列表（已去重並排序）
            </p>
          </div>
          <div class="p-4">
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">
              {#each lines as word, i}
                <div
                  class="flex items-center gap-2 p-2 bg-white rounded border text-sm"
                >
                  <span class="text-xs text-gray-400 w-6">{i + 1}</span>
                  <span class="font-mono text-gray-800">{word}</span>
                </div>
              {/each}
            </div>
          </div>
        </div>

        <!-- Import Button -->
        <div class="flex justify-center">
          <Button
            onclick={handleImport}
            disabled={!canImport}
            size="lg"
            class="px-8"
          >
            {#if $isImporting}
              🔄 匯入中...
            {:else}
              🚀 開始匯入 ({lines.length} 個單字)
            {/if}
          </Button>
        </div>
      </section>
    {/if}

    <!-- Import Progress -->
    <ImportProgress />

    <!-- Import Results -->
    <ImportResults />

    <!-- Warning for large files -->
    {#if lines.length > 100}
      <Alert.Root class="border-amber-200 bg-amber-50">
        <AlertTriangle class="h-4 w-4" />
        <Alert.Title class="text-amber-800">注意</Alert.Title>
        <Alert.Description class="text-amber-700">
          您即將匯入 {lines.length} 個單字，這可能需要較長時間。 建議分批匯入以獲得更好的體驗。
        </Alert.Description>
      </Alert.Root>
    {/if}

    <!-- Navigation -->
    <div class="text-center pt-6 border-t">
      <p class="text-gray-500 mb-4">匯入完成後，您可以：</p>
      <div class="flex justify-center gap-4">
        <Button variant="outline">
          <a href="/" class="flex items-center gap-1">🔍 查詢單字</a>
        </Button>
        <Button variant="outline">
          <a href="/cards" class="flex items-center gap-1">🃏 瀏覽單字卡</a>
        </Button>
      </div>
    </div>
  </div>
</MainLayout>
