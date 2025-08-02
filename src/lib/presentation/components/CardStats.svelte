<script lang="ts">
  import { Card, CardContent, CardHeader, CardTitle } from '$lib/presentation/components/ui/card';
  import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '$lib/presentation/components/ui/collapsible';
  import type { WordCard } from '$lib/domain/types/wordCard';

  // Collapsible 狀態，預設為收合
  let isOpen = $state(false);

  interface Props {
    allCards: WordCard[];
  }

  let { allCards }: Props = $props();

  // 定義建議類型
  interface Suggestion {
    type: string;
    emoji: string;
    message: string;
    color: string;
  }

  // 使用 $derived rune 計算統計資料
  const stats = $derived({
    total: allCards.length,
    byFamiliarity: {
      0: allCards.filter(card => card.familiarity === 0).length,
      1: allCards.filter(card => card.familiarity === 1).length,
      2: allCards.filter(card => card.familiarity === 2).length,
      3: allCards.filter(card => card.familiarity === 3).length,
    } as Record<number, number>,
    totalSeenCount: allCards.reduce((sum, card) => sum + card.seen_count, 0),
    averageFamiliarity: allCards.length > 0 
      ? (allCards.reduce((sum, card) => sum + card.familiarity, 0) / allCards.length).toFixed(1)
      : '0.0'
  });

  // 熟悉度等級定義
  const familiarityLevels = [
    { level: 0, label: '不熟悉', color: 'text-red-600', bgColor: 'bg-red-400', emoji: '😰' },
    { level: 1, label: '稍微熟悉', color: 'text-orange-600', bgColor: 'bg-orange-400', emoji: '🤔' },
    { level: 2, label: '熟悉', color: 'text-yellow-600', bgColor: 'bg-yellow-400', emoji: '😊' },
    { level: 3, label: '非常熟悉', color: 'text-green-600', bgColor: 'bg-green-400', emoji: '😎' }
  ];

  // 計算百分比
  function getPercentage(count: number): string {
    if (stats.total === 0) return '0%';
    return Math.round((count / stats.total) * 100) + '%';
  }

  // 獲取進度條寬度
  function getProgressWidth(count: number): string {
    if (stats.total === 0) return '0%';
    return Math.round((count / stats.total) * 100) + '%';
  }

  // 學習建議
  const suggestions = $derived((): Suggestion[] => {
    const suggestions: Suggestion[] = [];
    
    if (stats.byFamiliarity[0] > 0) {
      suggestions.push({
        type: 'warning',
        emoji: '⚠️',
        message: `有 ${stats.byFamiliarity[0]} 張不熟悉的單字卡需要加強`,
        color: 'text-red-600'
      });
    }
    
    if (stats.byFamiliarity[1] > 0) {
      suggestions.push({
        type: 'info',
        emoji: '📖',
        message: `有 ${stats.byFamiliarity[1]} 張稍微熟悉的單字卡可以複習`,
        color: 'text-orange-600'
      });
    }
    
    if (stats.byFamiliarity[3] > stats.total * 0.7) {
      suggestions.push({
        type: 'success',
        emoji: '🎉',
        message: '太棒了！大部分單字都很熟悉',
        color: 'text-green-600'
      });
    }
    
    if (stats.total < 10) {
      suggestions.push({
        type: 'tip',
        emoji: '💡',
        message: '建議多匯入一些單字來擴充詞彙量',
        color: 'text-blue-600'
      });
    }
    
    return suggestions;
  });
</script>

<Collapsible bind:open={isOpen}>
  <Card>
    <CardHeader>
      <CollapsibleTrigger class="w-full">
        <CardTitle class="flex items-center justify-between w-full">
          <div class="flex items-center space-x-2">
            <span>📊</span>
            <span>學習統計</span>
          </div>
          <span class="text-sm text-gray-500 transition-transform duration-200 {isOpen ? 'rotate-180' : ''}">
            ▼
          </span>
        </CardTitle>
      </CollapsibleTrigger>
    </CardHeader>
    
    <CollapsibleContent>
      <CardContent class="space-y-4">
        <!-- 總體統計 -->
        <div class="grid grid-cols-2 gap-3">
          <div class="text-center p-3 bg-blue-50 rounded-lg">
            <div class="text-2xl font-bold text-blue-600">{stats.total}</div>
            <div class="text-xs text-blue-800">總單字卡</div>
          </div>
          <div class="text-center p-3 bg-purple-50 rounded-lg">
            <div class="text-2xl font-bold text-purple-600">{stats.averageFamiliarity}</div>
            <div class="text-xs text-purple-800">平均熟悉度</div>
          </div>
        </div>

        <!-- 熟悉度分布 -->
        <div>
          <h4 class="text-sm font-medium text-gray-700 mb-3">熟悉度分布</h4>
          <div class="space-y-2">
            {#each familiarityLevels as level}
              {@const count = stats.byFamiliarity[level.level]}
              {@const percentage = getPercentage(count)}
              {@const progressWidth = getProgressWidth(count)}
              
              <div class="space-y-1">
                <!-- 標籤和數量 -->
                <div class="flex items-center justify-between text-sm">
                  <div class="flex items-center space-x-2">
                    <span>{level.emoji}</span>
                    <span class="text-gray-700">{level.label}</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <span class="font-medium {level.color}">{count}</span>
                    <span class="text-gray-500">({percentage})</span>
                  </div>
                </div>
                
                <!-- 進度條 -->
                <div class="w-full bg-gray-200 rounded-full h-2">
                  <div 
                    class="h-2 rounded-full transition-all duration-300 {level.bgColor}"
                    style="width: {progressWidth}"
                  ></div>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- 學習活動 -->
        <div class="pt-3 border-t">
          <h4 class="text-sm font-medium text-gray-700 mb-2">學習活動</h4>
          <div class="text-sm text-gray-600 space-y-1">
            <div class="flex justify-between">
              <span>總查看次數:</span>
              <span class="font-medium">{stats.totalSeenCount}</span>
            </div>
            <div class="flex justify-between">
              <span>平均查看次數:</span>
              <span class="font-medium">
                {stats.total > 0 ? (stats.totalSeenCount / stats.total).toFixed(1) : '0.0'}
              </span>
            </div>
          </div>
        </div>

        <!-- 學習建議 -->
        {#if stats.total > 0 && suggestions().length > 0}
          <div class="pt-3 border-t">
            <h4 class="text-sm font-medium text-gray-700 mb-2">學習建議</h4>
            <div class="text-xs text-gray-600 space-y-1">
              {#each suggestions() as suggestion}
                <div class="flex items-center space-x-2 {suggestion.color}">
                  <span>{suggestion.emoji}</span>
                  <span>{suggestion.message}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </CardContent>
    </CollapsibleContent>
  </Card>
</Collapsible>
