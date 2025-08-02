<script lang="ts">
  import { Card, CardContent, CardHeader, CardTitle } from '$lib/presentation/components/ui/card';
  import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '$lib/presentation/components/ui/collapsible';
  import { Button } from '$lib/presentation/components/ui/button';
  import type { WordCard } from '$lib/domain/types/wordCard';
  import { parsePronunciation } from '$lib/presentation/utils';

  interface Props {
    card: WordCard | null;
    onupdatefamiliarity: (familiarity: number) => void;
    onflip: () => void;
    ondelete: () => void;
    isFlipped?: boolean;
  }

  let { card, onupdatefamiliarity, onflip, ondelete, isFlipped = false }: Props = $props();

  // Collapsible 狀態，預設為收合
  let isOpen = $state(false);
  let isPlaying = $state(false);

  // 使用 $derived rune 進行響應式計算
  const pronunciation = $derived(card ? parsePronunciation(card.pronunciation) : null);

  // 熟悉度等級定義
  const familiarityLevels = [
    {
      level: 0,
      label: '不熟悉',
      color: 'bg-red-100 text-red-800',
      emoji: '😰',
    },
    {
      level: 1,
      label: '稍微熟悉',
      color: 'bg-orange-100 text-orange-800',
      emoji: '🤔',
    },
    {
      level: 2,
      label: '熟悉',
      color: 'bg-yellow-100 text-yellow-800',
      emoji: '😊',
    },
    {
      level: 3,
      label: '非常熟悉',
      color: 'bg-green-100 text-green-800',
      emoji: '😎',
    },
  ];

  const currentFamiliarity = $derived(
    card ? (familiarityLevels[card.familiarity] || familiarityLevels[0]) : familiarityLevels[0]
  );

  // 播放發音
  async function playPronunciation() {
    if (!pronunciation?.audio || isPlaying) return;

    try {
      isPlaying = true;
      const audio = new Audio(pronunciation.audio);
      audio.onended = () => (isPlaying = false);
      audio.onerror = () => (isPlaying = false);
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

  // 翻轉卡片
  function flipCard() {
    onflip();
  }

  // 刪除卡片
  function deleteCard() {
    ondelete();
  }
</script>

<Collapsible bind:open={isOpen}>
  <Card>
    <CardHeader>
      <CollapsibleTrigger class="w-full">
        <CardTitle class="flex items-center justify-between w-full">
          <div class="flex items-center space-x-2">
            <span>🎯</span>
            <span>熟悉度控制</span>
          </div>
          <span class="text-sm text-gray-500 transition-transform duration-200 {isOpen ? 'rotate-180' : ''}">
            ▼
          </span>
        </CardTitle>
      </CollapsibleTrigger>
    </CardHeader>
    
    <CollapsibleContent>
      <CardContent class="space-y-4">
        {#if card}
          <!-- 當前熟悉度顯示 -->
          <div class="text-center">
            <div
              class="inline-flex items-center space-x-2 px-4 py-2 rounded-full {currentFamiliarity.color}"
            >
              <span class="text-lg">{currentFamiliarity.emoji}</span>
              <span class="font-medium">{currentFamiliarity.label}</span>
            </div>
          </div>

          <!-- 熟悉度按鈕 -->
          <div class="grid grid-cols-2 gap-2">
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
          <div class="space-y-2 pt-4 border-t">
            <Button
              variant="outline"
              size="sm"
              onclick={flipCard}
              class="w-full flex items-center justify-center space-x-2"
            >
              <span>🔄</span>
              <span>{isFlipped ? '看單字' : '看定義'}</span>
            </Button>

            {#if pronunciation?.audio}
              <Button
                variant="outline"
                size="sm"
                onclick={playPronunciation}
                disabled={isPlaying}
                class="w-full flex items-center justify-center space-x-2"
              >
                <span>{isPlaying ? '⏸️' : '🔊'}</span>
                <span>發音</span>
              </Button>
            {/if}

            <Button
              variant="destructive"
              size="sm"
              onclick={deleteCard}
              class="w-full flex items-center justify-center space-x-2"
            >
              <span>🗑️</span>
              <span>刪除</span>
            </Button>
          </div>
        {:else}
          <!-- 無卡片狀態 -->
          <div class="text-center py-8 text-gray-500">
            <div class="text-4xl mb-2">🃏</div>
            <p class="text-sm">選擇一張單字卡來控制熟悉度</p>
          </div>
        {/if}
      </CardContent>
    </CollapsibleContent>
  </Card>
</Collapsible>
