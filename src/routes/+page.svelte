<script lang="ts">
  import { onMount } from 'svelte';
  import * as Alert from '$lib/presentation/components/ui/alert/index.js';
  import {
    SearchInput,
    WordDisplay,
    LoadingSpinner,
    ErrorDisplay,
  } from '$lib/presentation/components';
  import { MainLayout } from '$lib/presentation/layouts';
  import {
    dictionaryStore,
    hasEntry,
    canSaveCard,
  } from '$lib/application/stores';
  import {
    SearchCommand,
    SaveWordCardCommand,
  } from '$lib/application/commands';

  // Reactive store subscriptions
  let storeValue = $derived($dictionaryStore);
  let { query, entry, loading, error, existsInCard } = $derived(storeValue);
  let showEntry = $derived($hasEntry);
  let canSave = $derived($canSaveCard);

  // Event handlers
  async function handleSearch(query: string) {
    dictionaryStore.setQuery(query);
    await SearchCommand.execute(query);
  }

  async function handleSave() {
    await SaveWordCardCommand.execute();
  }

  function handleRetry() {
    if (query.trim()) {
      SearchCommand.execute(query);
    }
  }

  // Cleanup on component destroy
  onMount(() => {
    return () => {
      dictionaryStore.reset();
    };
  });
</script>

<MainLayout
  title="字典查詢 - 單字卡工具"
  description="查詢英文單字定義、發音，並建立個人單字卡"
>
  <div class="space-y-6 max-w-4xl mx-auto">
    <!-- Page Header -->
    <header class="text-center space-y-2">
      <h1 class="text-3xl font-bold text-gray-800">📖 字典查詢</h1>
      <p class="text-gray-600">輸入英文單字，查看詳細定義與發音</p>
    </header>

    <!-- Existing Word Alert -->
    {#if existsInCard}
      <Alert.Root class="border-amber-200 bg-amber-50">
        <Alert.Title class="text-amber-800">提示</Alert.Title>
        <Alert.Description class="text-amber-700">
          此單字已存在於單字卡中
        </Alert.Description>
      </Alert.Root>
    {/if}

    <!-- Search Section -->
    <section class="space-y-4">
      <SearchInput
        {query}
        {loading}
        {canSave}
        onsearch={handleSearch}
        onsave={handleSave}
      />
    </section>

    <!-- Content Section -->
    <section class="space-y-4">
      {#if loading}
        <LoadingSpinner message="正在查詢單字..." />
      {:else if error}
        <ErrorDisplay {error} onRetry={handleRetry} />
      {:else if showEntry && entry}
        <WordDisplay {entry} />
      {:else if query.trim()}
        <div class="text-center text-gray-500 py-8">
          <div class="text-4xl mb-4">🔍</div>
          <p class="text-lg">未找到相關結果</p>
          <p class="text-sm mt-2">請嘗試其他單字或檢查拼寫</p>
        </div>
      {:else}
        <div class="text-center text-gray-400 py-12">
          <div class="text-6xl mb-4">📚</div>
          <p class="text-lg font-medium">輸入單字開始查詢</p>
          <p class="text-sm mt-2">支援英文單字查詢與發音播放</p>
          <div class="mt-6 text-xs text-gray-400 space-y-1">
            <p>💡 提示：按 Enter 鍵快速搜尋</p>
            <p>🔊 支援發音播放功能</p>
            <p>💾 一鍵加入個人單字卡</p>
          </div>
        </div>
      {/if}
    </section>
  </div>
</MainLayout>
