<script lang="ts">
  import { onMount } from 'svelte';
  import MainLayout from '$lib/presentation/layouts/MainLayout.svelte';
  import FlashCard from '$lib/presentation/components/FlashCard.svelte';
  import CardFilters from '$lib/presentation/components/CardFilters.svelte';
  import CardStats from '$lib/presentation/components/CardStats.svelte';
  import { WordCardService } from '$lib/application/services/wordCardService';
  import type { WordCard } from '$lib/domain/types/wordCard';
  import { toast } from 'svelte-sonner';

  // 使用 Svelte 5 的 $state rune
  let allCards = $state<WordCard[]>([]);
  let filteredCards = $state<WordCard[]>([]);
  let currentCardIndex = $state(0);
  let isLoading = $state(true);
  let searchQuery = $state('');
  let selectedFamiliarity = $state<number | null>(null);
  let sortBy = $state<'word' | 'familiarity' | 'created_at'>('word');
  let sortOrder = $state<'asc' | 'desc'>('asc');

  // 使用 $derived rune 進行響應式計算
  const currentCard = $derived(filteredCards[currentCardIndex] || null);
  const hasCards = $derived(filteredCards.length > 0);
  const cardPosition = $derived(hasCards ? `${currentCardIndex + 1} / ${filteredCards.length}` : '0 / 0');

  // 載入所有單字卡
  async function loadAllCards() {
    try {
      isLoading = true;
      allCards = await WordCardService.getAllWordCards();
      applyFilters();
    } catch (error) {
      console.error('Failed to load cards:', error);
      toast.error('載入單字卡失敗');
    } finally {
      isLoading = false;
    }
  }

  // 應用篩選條件
  function applyFilters() {
    let filtered = [...allCards];

    // 搜尋篩選
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(card => 
        card.word.toLowerCase().includes(query) ||
        card.definition.toLowerCase().includes(query)
      );
    }

    // 熟悉度篩選
    if (selectedFamiliarity !== null) {
      filtered = filtered.filter(card => card.familiarity === selectedFamiliarity);
    }

    // 排序
    filtered.sort((a, b) => {
      let aValue: any, bValue: any;
      
      switch (sortBy) {
        case 'word':
          aValue = a.word.toLowerCase();
          bValue = b.word.toLowerCase();
          break;
        case 'familiarity':
          aValue = a.familiarity;
          bValue = b.familiarity;
          break;
        case 'created_at':
          aValue = new Date(a.created_at || 0);
          bValue = new Date(b.created_at || 0);
          break;
        default:
          return 0;
      }

      if (aValue < bValue) return sortOrder === 'asc' ? -1 : 1;
      if (aValue > bValue) return sortOrder === 'asc' ? 1 : -1;
      return 0;
    });

    filteredCards = filtered;
    currentCardIndex = 0; // 重置到第一張卡片
  }

  // 導航函數
  function nextCard() {
    if (currentCardIndex < filteredCards.length - 1) {
      currentCardIndex++;
    }
  }

  function prevCard() {
    if (currentCardIndex > 0) {
      currentCardIndex--;
    }
  }

  function goToCard(index: number) {
    if (index >= 0 && index < filteredCards.length) {
      currentCardIndex = index;
    }
  }

  // 更新熟悉度
  async function updateFamiliarity(cardId: number, newFamiliarity: number) {
    try {
      await WordCardService.updateFamiliarity(cardId, newFamiliarity);
      
      // 更新本地狀態
      const cardIndex = allCards.findIndex(card => card.id === cardId);
      if (cardIndex !== -1) {
        allCards[cardIndex].familiarity = newFamiliarity;
        applyFilters(); // 重新應用篩選
      }
      
      toast.success('熟悉度已更新');
    } catch (error) {
      console.error('Failed to update familiarity:', error);
      toast.error('更新熟悉度失敗');
    }
  }

  // 刪除單字卡
  async function deleteCard(cardId: number) {
    if (!confirm('確定要刪除這張單字卡嗎？')) return;
    
    try {
      await WordCardService.deleteWordCard(cardId);
      
      // 更新本地狀態
      allCards = allCards.filter(card => card.id !== cardId);
      applyFilters();
      
      // 調整當前卡片索引
      if (currentCardIndex >= filteredCards.length && filteredCards.length > 0) {
        currentCardIndex = filteredCards.length - 1;
      }
      
      toast.success('單字卡已刪除');
    } catch (error) {
      console.error('Failed to delete card:', error);
      toast.error('刪除單字卡失敗');
    }
  }

  // 鍵盤快捷鍵
  function handleKeydown(event: KeyboardEvent) {
    switch (event.key) {
      case 'ArrowLeft':
        event.preventDefault();
        prevCard();
        break;
      case 'ArrowRight':
        event.preventDefault();
        nextCard();
        break;
      case '1':
      case '2':
      case '3':
      case '4':
        if (currentCard) {
          const familiarity = parseInt(event.key) - 1;
          updateFamiliarity(currentCard.id!, familiarity);
        }
        break;
    }
  }

  // 使用 $effect rune 監聽篩選條件變化
  $effect(() => {
    applyFilters();
  });

  onMount(() => {
    loadAllCards();
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<MainLayout title="單字卡 - 單字卡工具" description="瀏覽和複習你的單字卡">
  <div class="max-w-6xl mx-auto space-y-6">
    <!-- 頁面標題 -->
    <div class="text-center">
      <h1 class="text-3xl font-bold text-gray-900 mb-2">🃏 單字卡</h1>
      <p class="text-gray-600">瀏覽、複習和管理你的單字卡</p>
    </div>

    {#if isLoading}
      <!-- 載入狀態 -->
      <div class="flex justify-center items-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <span class="ml-3 text-gray-600">載入中...</span>
      </div>
    {:else if allCards.length === 0}
      <!-- 空狀態 -->
      <div class="text-center py-12">
        <div class="text-6xl mb-4">📚</div>
        <h3 class="text-xl font-semibold text-gray-900 mb-2">還沒有單字卡</h3>
        <p class="text-gray-600 mb-6">開始查詢單字或匯入文字檔來建立你的第一張單字卡吧！</p>
        <div class="space-x-4">
          <a 
            href="/" 
            class="inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            🔍 字典查詢
          </a>
          <a 
            href="/import" 
            class="inline-flex items-center px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
          >
            📥 批次匯入
          </a>
        </div>
      </div>
    {:else}
      <!-- 主要內容 -->
      <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
        <!-- 左側：篩選和統計 -->
        <div class="lg:col-span-1 space-y-4">
          <CardStats {allCards} />
          <CardFilters 
            bind:searchQuery
            bind:selectedFamiliarity
            bind:sortBy
            bind:sortOrder
            totalCards={allCards.length}
            filteredCards={filteredCards.length}
          />
        </div>

        <!-- 右側：單字卡展示 -->
        <div class="lg:col-span-3">
          {#if hasCards && currentCard}
            <div class="space-y-4">
              <!-- 卡片位置指示器 -->
              <div class="flex justify-between items-center">
                <div class="text-sm text-gray-500">
                  {cardPosition}
                </div>
                <div class="text-xs text-gray-400">
                  使用 ← → 鍵導航，按 1-4 鍵設定熟悉度
                </div>
              </div>

              <!-- 單字卡 -->
              <FlashCard 
                card={currentCard}
                onupdatefamiliarity={(familiarity) => updateFamiliarity(currentCard.id!, familiarity)}
                ondelete={() => deleteCard(currentCard.id!)}
              />

              <!-- 導航控制 -->
              <div class="flex justify-center items-center space-x-4">
                <button
                  onclick={prevCard}
                  disabled={currentCardIndex === 0}
                  class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                  ← 上一張
                </button>
                
                <span class="text-sm text-gray-500 min-w-[80px] text-center">
                  {cardPosition}
                </span>
                
                <button
                  onclick={nextCard}
                  disabled={currentCardIndex === filteredCards.length - 1}
                  class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                  下一張 →
                </button>
              </div>

              <!-- 快速跳轉 -->
              {#if filteredCards.length > 1}
                <div class="flex justify-center">
                  <div class="flex space-x-1 max-w-full overflow-x-auto">
                    {#each filteredCards as card, index}
                      <button
                        onclick={() => goToCard(index)}
                        class="w-8 h-8 text-xs rounded-full border-2 transition-colors {
                          index === currentCardIndex 
                            ? 'bg-blue-600 text-white border-blue-600' 
                            : 'bg-white text-gray-600 border-gray-300 hover:border-blue-400'
                        }"
                        title={card.word}
                      >
                        {index + 1}
                      </button>
                    {/each}
                  </div>
                </div>
              {/if}
            </div>
          {:else}
            <!-- 篩選後無結果 -->
            <div class="text-center py-12">
              <div class="text-4xl mb-4">🔍</div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">找不到符合條件的單字卡</h3>
              <p class="text-gray-600">試試調整篩選條件或搜尋關鍵字</p>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</MainLayout>
