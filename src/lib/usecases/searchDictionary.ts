// src/lib/usecases/searchDictionary.ts

import { fetchDictionaryEntry } from '$lib/infrastructure/dictionaryApi';
import type { DictionaryEntry } from '$lib/domain/dictionary';

/**
 * 查詢字典詞條，如果找不到，會拋出錯誤。
 */
export async function searchWord(word: string): Promise<DictionaryEntry[]> {
  const result = await fetchDictionaryEntry(word);

  if (result.success) {
    return result.entries;
  }

  // Application Layer 可選擇將錯誤語意拋出，由上層 UI 捕捉
  if (result.reason === 'not_found') {
    throw new Error(`找不到單字 "${word}" 的定義`);
  } else {
    throw new Error(`查詢失敗：${result.message}`);
  }
}
