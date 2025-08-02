<script lang="ts">
  import type { WordCard } from '$lib/domain/types/wordCard';
  import { parsePronunciation } from '$lib/presentation/utils';
  import { Button } from '$lib/presentation/components/ui/button';
  import { Card, CardContent, CardHeader } from '$lib/presentation/components/ui/card';

  interface Props {
    card: WordCard;
    onupdatefamiliarity: (familiarity: number) => void;
    ondelete: () => void;
  }

  let { card, onupdatefamiliarity, ondelete }: Props = $props();

  let isFlipped = $state(false);
  let isPlaying = $state(false);

  // 使用 $derived rune 進行響應式計算
  const pronunciation = $derived(parsePronunciation(card.pronunciation));
  const posArray = $derived(card.pos ? JSON.parse(card.pos) : []);

  // 熟悉度等級定義
  const familiarityLevels = [
    { level: 0, label: '不熟悉', color: 'bg-red-100 text-red-800', emoji: '😰' },
    { level: 1, label: '稍微熟悉', color: 'bg-orange-100 text-orange-800', emoji: '🤔' },
    { level: 2, label: '熟悉', color: 'bg-yellow-100 text-yellow-800', emoji: '😊' },
    { level: 3, label: '非常熟悉', color: 'bg-green-100 text-green-800', emoji: '😎' }
  ];

  const currentFamiliarity = $derived(familiarityLevels[card.familiarity] || familiarityLevels[0]);

  // 翻轉卡片
  function flipCard() {
    isFlipped = !isFlipped;
  }

  // 播放發音
  async function playPronunciation() {
    if (!pronunciation?.audio || isPlaying) return;
    
    try {
      isPlaying = true;
      const audio = new Audio(pronunciation.audio);
      audio.onended = () => isPlaying = false;
      audio.onerror = () => isPlaying = false;
      await audio.play();
    } catch (error) {
      console.error('Failed to play audio:', error);
      isPlaying = false;
    }
  }

  // 更新熟悉度
  function updateFamiliarity(level: number) {
    onupdatefamiliarity(level);
  }

  // 刪除卡片
  function deleteCard() {
    ondelete();
  }

  // 格式化日期
  function formatDate(dateString?: string) {
    if (!dateString) return '';
    return new Date(dateString).toLocaleDateString('zh-TW');
  }
</script>

<div class="max-w-2xl mx-auto">
  <!-- 卡片容器 -->
  <div class="relative perspective-1000">
    <div 
      class="card-container {isFlipped ? 'flipped' : ''}"
      onclick={flipCard}
      onkeydown={(e) => e.key === ' ' && flipCard()}
      role="button"
      tabindex="0"
    >
      <!-- 正面 (單字) -->
      <Card class="card-face card-front">
        <CardHeader class="text-center pb-4">
          <div class="flex items-center justify-center space-x-2 mb-2">
            <h2 class="text-4xl font-bold text-gray-900">{card.word}</h2>
            {#if pronunciation?.audio}
              <button
                onclick={(e) => { e.stopPropagation(); playPronunciation(); }}
                disabled={isPlaying}
                class="p-2 rounded-full hover:bg-gray-100 transition-colors {isPlaying ? 'animate-pulse' : ''}"
                title="播放發音"
              >
                🔊
              </button>
            {/if}
          </div>
          
          {#if pronunciation?.phonetic}
            <p class="text-lg text-gray-600 font-mono">/{pronunciation.phonetic}/</p>
          {/if}
          
          {#if posArray.length > 0}
            <div class="flex justify-center space-x-2 mt-2">
              {#each posArray as pos}
                <span class="px-2 py-1 bg-blue-100 text-blue-800 text-sm rounded-full">
                  {pos}
                </span>
              {/each}
            </div>
          {/if}
        </CardHeader>
        
        <CardContent class="text-center">
          <p class="text-gray-500 mb-4">點擊查看定義</p>
          <div class="text-6xl mb-4">🤔</div>
        </CardContent>
      </Card>

      <!-- 背面 (定義) -->
      <Card class="card-face card-back">
        <CardHeader class="text-center pb-4">
          <h3 class="text-2xl font-bold text-gray-900 mb-2">{card.word}</h3>
          {#if pronunciation?.phonetic}
            <p class="text-gray-600 font-mono">/{pronunciation.phonetic}/</p>
          {/if}
        </CardHeader>
        
        <CardContent>
          <div class="space-y-4">
            <!-- 定義 -->
            <div class="bg-gray-50 p-4 rounded-lg">
              <h4 class="font-semibold text-gray-900 mb-2">定義</h4>
              <p class="text-gray-700 leading-relaxed">{card.definition}</p>
            </div>

            <!-- 詞性 -->
            {#if posArray.length > 0}
              <div>
                <h4 class="font-semibold text-gray-900 mb-2">詞性</h4>
                <div class="flex flex-wrap gap-2">
                  {#each posArray as pos}
                    <span class="px-3 py-1 bg-blue-100 text-blue-800 text-sm rounded-full">
                      {pos}
                    </span>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- 統計資訊 -->
            <div class="text-sm text-gray-500 border-t pt-3">
              <div class="flex justify-between">
                <span>查看次數: {card.seen_count}</span>
                <span>建立時間: {formatDate(card.created_at)}</span>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>

  <!-- 熟悉度控制 -->
  <div class="mt-6 space-y-4">
    <!-- 當前熟悉度顯示 -->
    <div class="text-center">
      <div class="inline-flex items-center space-x-2 px-4 py-2 rounded-full {currentFamiliarity.color}">
        <span class="text-lg">{currentFamiliarity.emoji}</span>
        <span class="font-medium">{currentFamiliarity.label}</span>
      </div>
    </div>

    <!-- 熟悉度按鈕 -->
    <div class="grid grid-cols-4 gap-2">
      {#each familiarityLevels as level}
        <Button
          variant={card.familiarity === level.level ? 'default' : 'outline'}
          size="sm"
          onclick={() => updateFamiliarity(level.level)}
          class="flex flex-col items-center py-3 h-auto"
        >
          <span class="text-lg mb-1">{level.emoji}</span>
          <span class="text-xs">{level.label}</span>
          <span class="text-xs text-gray-500">({level.level + 1})</span>
        </Button>
      {/each}
    </div>

    <!-- 操作按鈕 -->
    <div class="flex justify-center space-x-3 pt-4 border-t">
      <Button
        variant="outline"
        size="sm"
        onclick={flipCard}
        class="flex items-center space-x-2"
      >
        <span>🔄</span>
        <span>翻轉</span>
      </Button>
      
      {#if pronunciation?.audio}
        <Button
          variant="outline"
          size="sm"
          onclick={playPronunciation}
          disabled={isPlaying}
          class="flex items-center space-x-2"
        >
          <span>{isPlaying ? '⏸️' : '🔊'}</span>
          <span>發音</span>
        </Button>
      {/if}
      
      <Button
        variant="destructive"
        size="sm"
        onclick={deleteCard}
        class="flex items-center space-x-2"
      >
        <span>🗑️</span>
        <span>刪除</span>
      </Button>
    </div>
  </div>
</div>

<style>
  .perspective-1000 {
    perspective: 1000px;
  }

  .card-container {
    position: relative;
    width: 100%;
    height: 400px;
    transform-style: preserve-3d;
    transition: transform 0.6s;
    cursor: pointer;
  }

  .card-container.flipped {
    transform: rotateY(180deg);
  }

  .card-face {
    position: absolute;
    width: 100%;
    height: 100%;
    backface-visibility: hidden;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .card-front {
    z-index: 2;
  }

  .card-back {
    transform: rotateY(180deg);
  }

  .card-container:hover {
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
  }
</style>
