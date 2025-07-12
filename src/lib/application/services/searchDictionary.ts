// ✅ src/lib/usecases/searchDictionary.ts

import type { DictionaryEntry } from '$lib/domain/models';
import { fetchDictionaryEntry } from '$lib/infrastructure/api/dictionaryService';

/**
 * 查詢字典詞條，如果找不到，會拋出錯誤。
 * 回傳已整理過的 DictionaryEntry。
 */
export async function searchWord(word: string): Promise<DictionaryEntry> {
  const entry = await fetchDictionaryEntry(word);

  if (!entry) {
    throw new Error(`找不到 "${word}" 的定義`);
  }

  return entry;
}
