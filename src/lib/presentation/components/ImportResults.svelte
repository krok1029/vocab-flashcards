<script lang="ts">
  import { importStore } from '$lib/application/stores';
  import { Badge } from '$lib/presentation/components/ui/badge';
  import { Button } from '$lib/presentation/components/ui/button';
  import * as Collapsible from '$lib/presentation/components/ui/collapsible';
  import { ChevronDown, ChevronRight, CircleCheck, CircleX, CircleAlert } from '@lucide/svelte';

  let { results, successCount, failedCount, existsCount } = $derived($importStore);
  
  let showSuccess = $state(false);
  let showExists = $state(false);
  let showFailed = $state(true); // 預設展開失敗項目

  let successResults = $derived(results.filter(r => r.status === 'success'));
  let existsResults = $derived(results.filter(r => r.status === 'exists'));
  let failedResults = $derived(results.filter(r => r.status === 'failed'));

  function handleReset() {
    importStore.reset();
  }

  function toggleSuccess() {
    showSuccess = !showSuccess;
  }

  function toggleExists() {
    showExists = !showExists;
  }

  function toggleFailed() {
    showFailed = !showFailed;
  }
</script>

{#if results.length > 0}
  <div class="space-y-4 p-6 bg-gray-50 border rounded-lg">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-gray-800">
        📊 匯入結果
      </h3>
      <Button 
        variant="outline" 
        size="sm" 
        onclick={handleReset}
      >
        重新開始
      </Button>
    </div>

    <!-- 統計摘要 -->
    <div class="grid grid-cols-3 gap-4 text-sm">
      <div class="text-center p-3 bg-green-100 rounded-lg">
        <div class="text-2xl font-bold text-green-800">{successCount}</div>
        <div class="text-green-600">成功匯入</div>
      </div>
      <div class="text-center p-3 bg-yellow-100 rounded-lg">
        <div class="text-2xl font-bold text-yellow-800">{existsCount}</div>
        <div class="text-yellow-600">已存在</div>
      </div>
      <div class="text-center p-3 bg-red-100 rounded-lg">
        <div class="text-2xl font-bold text-red-800">{failedCount}</div>
        <div class="text-red-600">匯入失敗</div>
      </div>
    </div>

    <!-- 詳細結果 -->
    <div class="space-y-2">
      <!-- 成功項目 -->
      {#if successResults.length > 0}
        <Collapsible.Root bind:open={showSuccess}>
          <Collapsible.Trigger class="w-full">
            <Button 
              variant="ghost" 
              class="w-full justify-between p-3 h-auto"
              onclick={toggleSuccess}
            >
              <div class="flex items-center gap-2">
                <CircleCheck class="w-4 h-4 text-green-600" />
                <span class="font-medium">成功匯入 ({successCount})</span>
              </div>
              {#if showSuccess}
                <ChevronDown class="w-4 h-4" />
              {:else}
                <ChevronRight class="w-4 h-4" />
              {/if}
            </Button>
          </Collapsible.Trigger>
          <Collapsible.Content class="space-y-1 px-3 pb-2">
            {#each successResults as result}
              <div class="flex items-center justify-between py-2 px-3 bg-green-50 rounded text-sm">
                <span class="font-mono">{result.word}</span>
                <Badge variant="secondary" class="bg-green-100 text-green-800">
                  已儲存
                </Badge>
              </div>
            {/each}
          </Collapsible.Content>
        </Collapsible.Root>
      {/if}

      <!-- 已存在項目 -->
      {#if existsResults.length > 0}
        <Collapsible.Root bind:open={showExists}>
          <Collapsible.Trigger class="w-full">
            <Button 
              variant="ghost" 
              class="w-full justify-between p-3 h-auto"
              onclick={toggleExists}
            >
              <div class="flex items-center gap-2">
                <CircleAlert class="w-4 h-4 text-yellow-600" />
                <span class="font-medium">已存在 ({existsCount})</span>
              </div>
              {#if showExists}
                <ChevronDown class="w-4 h-4" />
              {:else}
                <ChevronRight class="w-4 h-4" />
              {/if}
            </Button>
          </Collapsible.Trigger>
          <Collapsible.Content class="space-y-1 px-3 pb-2">
            {#each existsResults as result}
              <div class="flex items-center justify-between py-2 px-3 bg-yellow-50 rounded text-sm">
                <span class="font-mono">{result.word}</span>
                <Badge variant="secondary" class="bg-yellow-100 text-yellow-800">
                  跳過
                </Badge>
              </div>
            {/each}
          </Collapsible.Content>
        </Collapsible.Root>
      {/if}

      <!-- 失敗項目 -->
      {#if failedResults.length > 0}
        <Collapsible.Root bind:open={showFailed}>
          <Collapsible.Trigger class="w-full">
            <Button 
              variant="ghost" 
              class="w-full justify-between p-3 h-auto"
              onclick={toggleFailed}
            >
              <div class="flex items-center gap-2">
                <CircleX class="w-4 h-4 text-red-600" />
                <span class="font-medium">匯入失敗 ({failedCount})</span>
              </div>
              {#if showFailed}
                <ChevronDown class="w-4 h-4" />
              {:else}
                <ChevronRight class="w-4 h-4" />
              {/if}
            </Button>
          </Collapsible.Trigger>
          <Collapsible.Content class="space-y-1 px-3 pb-2">
            {#each failedResults as result}
              <div class="py-2 px-3 bg-red-50 rounded text-sm space-y-1">
                <div class="flex items-center justify-between">
                  <span class="font-mono font-medium">{result.word}</span>
                  <Badge variant="destructive" class="text-xs">
                    失敗
                  </Badge>
                </div>
                {#if result.error}
                  <div class="text-xs text-red-600 pl-2">
                    錯誤: {result.error}
                  </div>
                {/if}
              </div>
            {/each}
          </Collapsible.Content>
        </Collapsible.Root>
      {/if}
    </div>
  </div>
{/if}
