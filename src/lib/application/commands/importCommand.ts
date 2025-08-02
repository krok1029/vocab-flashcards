import { toast } from 'svelte-sonner';
import { importStore } from '$lib/application/stores/importStore';
import { searchWord } from '$lib/application/services/searchDictionary';
import { WordCardService } from '$lib/application/services/wordCardService';
import type { DictionaryEntry } from '$lib/domain/models/dictionary';

export interface ImportResult {
  word: string;
  status: 'success' | 'failed' | 'exists' | 'processing';
  entry?: DictionaryEntry;
  error?: string;
}

export class ImportCommand {
  static async execute(words: string[]): Promise<void> {
    if (words.length === 0) {
      toast.error('沒有單字需要匯入');
      return;
    }

    importStore.startImport(words.length);
    const results: ImportResult[] = [];

    for (let i = 0; i < words.length; i++) {
      const word = words[i];
      importStore.setCurrentWord(word, i + 1);

      try {
        // 檢查是否已存在
        const existingCard = await WordCardService.getWordCard(word);
        
        if (existingCard) {
          results.push({
            word,
            status: 'exists',
            entry: WordCardService.mapWordCardToDictionaryEntry(existingCard)
          });
          importStore.incrementExists();
          continue;
        }

        // 查詢字典 API
        const entry = await searchWord(word);
        
        // 儲存單字卡
        await WordCardService.saveWordCard(entry);
        
        results.push({
          word,
          status: 'success',
          entry
        });
        
        importStore.incrementSuccess();
        
        // 避免 API 請求過於頻繁
        await new Promise(resolve => setTimeout(resolve, 100));
        
      } catch (error) {
        results.push({
          word,
          status: 'failed',
          error: String(error)
        });
        importStore.incrementFailed();
      }
    }

    importStore.completeImport(results);
    
    const { successCount, failedCount, existsCount } = importStore.getStats();
    
    if (successCount > 0) {
      toast.success(`成功匯入 ${successCount} 個單字`);
    }
    
    if (failedCount > 0) {
      toast.error(`${failedCount} 個單字匯入失敗`);
    }
    
    if (existsCount > 0) {
      toast.info(`${existsCount} 個單字已存在`);
    }
  }

  static cancel(): void {
    importStore.cancelImport();
    toast.info('匯入已取消');
  }
}
