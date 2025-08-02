<script lang="ts">
  import { importStore, importProgress } from '$lib/application/stores';
  import { ImportCommand } from '$lib/application/commands';
  import { Button } from '$lib/presentation/components/ui/button';
  import { Progress } from '$lib/presentation/components/ui/progress';

  let { 
    isImporting, 
    currentWord, 
    currentIndex, 
    totalWords,
    successCount,
    failedCount,
    existsCount 
  } = $derived($importStore);

  function handleCancel() {
    ImportCommand.cancel();
  }
</script>

{#if isImporting}
  <div class="space-y-4 p-6 bg-blue-50 border border-blue-200 rounded-lg">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-blue-800">
        🔄 正在匯入單字...
      </h3>
      <Button 
        variant="outline" 
        size="sm" 
        onclick={handleCancel}
        class="text-red-600 border-red-300 hover:bg-red-50"
      >
        取消匯入
      </Button>
    </div>

    <div class="space-y-2">
      <div class="flex justify-between text-sm text-gray-600">
        <span>進度: {currentIndex} / {totalWords}</span>
        <span>{Math.round($importProgress)}%</span>
      </div>
      
      <Progress value={$importProgress} class="h-2" />
      
      {#if currentWord}
        <p class="text-sm text-blue-700">
          正在處理: <span class="font-mono font-medium">{currentWord}</span>
        </p>
      {/if}
    </div>

    <div class="grid grid-cols-3 gap-4 text-sm">
      <div class="text-center p-2 bg-green-100 rounded">
        <div class="font-semibold text-green-800">{successCount}</div>
        <div class="text-green-600">成功</div>
      </div>
      <div class="text-center p-2 bg-yellow-100 rounded">
        <div class="font-semibold text-yellow-800">{existsCount}</div>
        <div class="text-yellow-600">已存在</div>
      </div>
      <div class="text-center p-2 bg-red-100 rounded">
        <div class="font-semibold text-red-800">{failedCount}</div>
        <div class="text-red-600">失敗</div>
      </div>
    </div>
  </div>
{/if}
