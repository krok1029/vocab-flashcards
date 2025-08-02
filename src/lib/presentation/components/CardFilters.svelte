<script lang="ts">
  import { Card, CardContent, CardHeader, CardTitle } from '$lib/presentation/components/ui/card';
  import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '$lib/presentation/components/ui/collapsible';
  import { Input } from '$lib/presentation/components/ui/input';
  import { Button } from '$lib/presentation/components/ui/button';
  import { Label } from '$lib/presentation/components/ui/label';

  // Collapsible 狀態，預設為收合
  let isOpen = $state(false);

  interface Props {
    searchQuery: string;
    selectedFamiliarity: number | null;
    sortBy: 'word' | 'familiarity' | 'created_at';
    sortOrder: 'asc' | 'desc';
    totalCards: number;
    filteredCards: number;
  }

  let { 
    searchQuery = $bindable(),
    selectedFamiliarity = $bindable(),
    sortBy = $bindable(),
    sortOrder = $bindable(),
    totalCards,
    filteredCards
  }: Props = $props();

  // 熟悉度選項
  const familiarityOptions = [
    { value: null, label: '全部', emoji: '📚' },
    { value: 0, label: '不熟悉', emoji: '😰' },
    { value: 1, label: '稍微熟悉', emoji: '🤔' },
    { value: 2, label: '熟悉', emoji: '😊' },
    { value: 3, label: '非常熟悉', emoji: '😎' }
  ];

  // 排序選項
  const sortOptions: Array<{ value: 'word' | 'familiarity' | 'created_at'; label: string }> = [
    { value: 'word', label: '單字' },
    { value: 'familiarity', label: '熟悉度' },
    { value: 'created_at', label: '建立時間' }
  ];

  // 檢查是否有篩選條件
  const hasFilters = $derived(
    searchQuery || 
    selectedFamiliarity !== null || 
    sortBy !== 'word' || 
    sortOrder !== 'asc'
  );

  // 清除所有篩選
  function clearFilters() {
    searchQuery = '';
    selectedFamiliarity = null;
    sortBy = 'word';
    sortOrder = 'asc';
  }

  // 切換排序順序
  function toggleSortOrder() {
    sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
  }
</script>

<Collapsible bind:open={isOpen}>
  <Card>
    <CardHeader>
      <CollapsibleTrigger class="w-full">
        <CardTitle class="flex items-center justify-between w-full">
          <div class="flex items-center space-x-2">
            <span>🔍</span>
            <span>篩選與排序</span>
          </div>
          <span class="text-sm text-gray-500 transition-transform duration-200 {isOpen ? 'rotate-180' : ''}">
            ▼
          </span>
        </CardTitle>
      </CollapsibleTrigger>
    </CardHeader>
    
    <CollapsibleContent>
      <CardContent class="space-y-4">
        <!-- 搜尋框 -->
        <div>
          <Label for="search" class="text-sm font-medium text-gray-700">搜尋單字</Label>
          <Input
            id="search"
            type="text"
            placeholder="輸入單字或定義..."
            bind:value={searchQuery}
            class="mt-1"
          />
        </div>

        <!-- 熟悉度篩選 -->
        <div>
          <Label class="text-sm font-medium text-gray-700 mb-2 block">熟悉度</Label>
          <div class="grid grid-cols-1 gap-1">
            {#each familiarityOptions as option}
              <button
                onclick={() => selectedFamiliarity = option.value}
                class="flex items-center space-x-2 px-3 py-2 text-sm rounded-lg border transition-colors {
                  selectedFamiliarity === option.value
                    ? 'bg-blue-100 border-blue-300 text-blue-800'
                    : 'bg-white border-gray-200 text-gray-700 hover:bg-gray-50'
                }"
              >
                <span>{option.emoji}</span>
                <span>{option.label}</span>
              </button>
            {/each}
          </div>
        </div>

        <!-- 排序選項 -->
        <div>
          <Label class="text-sm font-medium text-gray-700 mb-2 block">排序方式</Label>
          <div class="space-y-2">
            <!-- 排序欄位 -->
            <div class="grid grid-cols-1 gap-1">
              {#each sortOptions as option}
                <button
                  onclick={() => sortBy = option.value}
                  class="flex items-center justify-between px-3 py-2 text-sm rounded-lg border transition-colors {
                    sortBy === option.value
                      ? 'bg-blue-100 border-blue-300 text-blue-800'
                      : 'bg-white border-gray-200 text-gray-700 hover:bg-gray-50'
                  }"
                >
                  <span>{option.label}</span>
                  {#if sortBy === option.value}
                    <span class="text-blue-600">✓</span>
                  {/if}
                </button>
              {/each}
            </div>

            <!-- 排序順序 -->
            <Button
              variant="outline"
              size="sm"
              onclick={toggleSortOrder}
              class="w-full flex items-center justify-center space-x-2"
            >
              <span>{sortOrder === 'asc' ? '⬆️' : '⬇️'}</span>
              <span>{sortOrder === 'asc' ? '升序' : '降序'}</span>
            </Button>
          </div>
        </div>

        <!-- 結果統計 -->
        <div class="pt-3 border-t">
          <div class="text-sm text-gray-600 space-y-1">
            <div class="flex justify-between">
              <span>總計:</span>
              <span class="font-medium">{totalCards} 張</span>
            </div>
            <div class="flex justify-between">
              <span>顯示:</span>
              <span class="font-medium text-blue-600">{filteredCards} 張</span>
            </div>
          </div>
        </div>

        <!-- 清除篩選 -->
        {#if hasFilters}
          <Button
            variant="outline"
            size="sm"
            onclick={clearFilters}
            class="w-full flex items-center justify-center space-x-2"
          >
            <span>🔄</span>
            <span>清除篩選</span>
          </Button>
        {/if}
      </CardContent>
    </CollapsibleContent>
  </Card>
</Collapsible>
