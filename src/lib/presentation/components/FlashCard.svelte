<script lang="ts">
  import type { WordCard } from '$lib/domain/types/wordCard';
  import { parsePronunciation } from '$lib/presentation/utils';
  import {
    Card,
    CardContent,
    CardHeader,
  } from '$lib/presentation/components/ui/card';

  interface Props {
    card: WordCard;
    onupdatefamiliarity: (familiarity: number) => void;
    ondelete: () => void;
    onflip?: () => void;
    isFlipped?: boolean;
  }

  let { card, onupdatefamiliarity, ondelete, onflip, isFlipped = false }: Props = $props();

  let internalFlipped = $state(false);

  // 使用 $derived rune 進行響應式計算
  const pronunciation = $derived(parsePronunciation(card.pronunciation));
  const posArray = $derived(card.pos ? JSON.parse(card.pos) : []);

  // 使用 $derived 來計算實際的翻轉狀態
  const actualFlipped = $derived(isFlipped !== undefined ? isFlipped : internalFlipped);

  // 解析定義文本為結構化資料
  const parsedDefinitions = $derived(
    (() => {
      if (!card.definition) return [];

      const definitionBlocks = card.definition.split('\n\n');
      return definitionBlocks.map((block) => {
        const lines = block.split('\n');
        const firstLine = lines[0];

        // 提取詞性和定義
        const posMatch = firstLine.match(/^\[([^\]]+)\]\s*(.+)$/);
        const partOfSpeech = posMatch ? posMatch[1] : '';
        const definition = posMatch ? posMatch[2] : firstLine;

        // 提取例句
        const exampleLine = lines.find((line) => line.startsWith('例句: '));
        const example = exampleLine ? exampleLine.replace('例句: ', '') : null;

        // 提取同義詞
        const synonymsLine = lines.find((line) => line.startsWith('同義詞: '));
        const synonyms = synonymsLine
          ? synonymsLine.replace('同義詞: ', '').split(', ')
          : [];

        // 提取反義詞
        const antonymsLine = lines.find((line) => line.startsWith('反義詞: '));
        const antonyms = antonymsLine
          ? antonymsLine.replace('反義詞: ', '').split(', ')
          : [];

        return {
          partOfSpeech,
          definition,
          example,
          synonyms,
          antonyms,
        };
      });
    })()
  );

  // 翻轉卡片
  function flipCard() {
    if (isFlipped !== undefined) {
      // 使用外部狀態
      onflip?.();
    } else {
      // 使用內部狀態
      internalFlipped = !internalFlipped;
    }
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
      class="group card-container {actualFlipped ? 'flipped' : ''}"
      onclick={flipCard}
      onkeydown={(e) => e.key === ' ' && flipCard()}
      role="button"
      tabindex="0"
    >
      <!-- 正面 (單字) -->
      <Card
        class="card-face card-front rounded-xl overflow-hidden shadow-lg bg-white group-[.flipped]:hidden"
      >
        <CardHeader class="text-center pb-4">
          <div class="flex items-center justify-center space-x-2 mb-2">
            <h2 class="text-4xl font-bold text-gray-900">{card.word}</h2>
          </div>

          {#if pronunciation?.phonetic}
            <p class="text-lg text-gray-600 font-mono">
              /{pronunciation.phonetic}/
            </p>
          {/if}

          {#if posArray.length > 0}
            <div class="flex justify-center space-x-2 mt-2">
              {#each posArray as pos}
                <span
                  class="px-2 py-1 bg-blue-100 text-blue-800 text-sm rounded-full"
                >
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
      <Card
        class="card-face card-back rounded-xl shadow-lg bg-white hidden group-[.flipped]:block"
      >
        <div class="transform rotate-y-180 flex flex-col">
          <CardHeader class="text-center pb-4 flex-shrink-0">
            <h3 class="text-2xl font-bold text-gray-900 mb-2">{card.word}</h3>
            {#if pronunciation?.phonetic}
              <p class="text-gray-600 font-mono">/{pronunciation.phonetic}/</p>
            {/if}
          </CardHeader>

          <CardContent class="flex-1 overflow-y-scroll">
            <div class="space-y-4">
              <!-- 詞性 -->
              {#if posArray.length > 0}
                <div>
                  <h4 class="font-semibold text-gray-900 mb-2">詞性</h4>
                  <div class="flex flex-wrap gap-2">
                    {#each posArray as pos}
                      <span
                        class="px-3 py-1 bg-blue-100 text-blue-800 text-sm rounded-full"
                      >
                        {pos}
                      </span>
                    {/each}
                  </div>
                </div>
              {/if}
              
              <!-- 定義 -->
              <div class="bg-gray-50 p-4 rounded-lg">
                <h4 class="font-semibold text-gray-900 mb-3">定義</h4>
                <div class="space-y-4 overflow-y-auto h-[500px]">
                  {#each parsedDefinitions as def, index}
                    <div class="border-l-4 border-blue-200 pl-4">
                      <!-- 詞性標籤 -->
                      {#if def.partOfSpeech}
                        <div class="mb-2">
                          <span
                            class="inline-block px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-full font-medium"
                          >
                            {def.partOfSpeech}
                          </span>
                        </div>
                      {/if}

                      <!-- 定義文本 -->
                      <p class="text-gray-700 leading-relaxed mb-2">
                        {def.definition}
                      </p>

                      <!-- 例句 -->
                      {#if def.example}
                        <div class="bg-blue-50 p-2 rounded text-sm">
                          <span class="text-blue-600 font-medium">例句：</span>
                          <span class="text-gray-700 italic">{def.example}</span>
                        </div>
                      {/if}

                      <!-- 同義詞和反義詞 -->
                      {#if def.synonyms.length > 0 || def.antonyms.length > 0}
                        <div class="mt-2 space-y-1">
                          {#if def.synonyms.length > 0}
                            <div class="text-xs">
                              <span class="text-green-600 font-medium">同義詞：</span>
                              <span class="text-gray-600">{def.synonyms.join(', ')}</span>
                            </div>
                          {/if}
                          {#if def.antonyms.length > 0}
                            <div class="text-xs">
                              <span class="text-red-600 font-medium">反義詞：</span>
                              <span class="text-gray-600">{def.antonyms.join(', ')}</span>
                            </div>
                          {/if}
                        </div>
                      {/if}
                    </div>

                    <!-- 分隔線（除了最後一個） -->
                    {#if index < parsedDefinitions.length - 1}
                      <hr class="border-gray-200" />
                    {/if}
                  {/each}
                </div>
              </div>

              <!-- 統計資訊 -->
              <div class="text-sm text-gray-500 border-t pt-3">
                <div class="flex justify-between">
                  <span>查看次數: {card.seen_count}</span>
                  <span>建立時間: {formatDate(card.created_at)}</span>
                </div>
              </div>
            </div>
          </CardContent>
        </div>
      </Card>
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
    transform-style: preserve-3d;
    transition: transform 0.6s ease;
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
    top: 0;
    left: 0;
    border-radius: 1rem;
  }

  .card-front {
    z-index: 2;
    transform: rotateY(0deg);
  }

  .card-back {
    transform: rotateY(180deg);
  }
</style>
