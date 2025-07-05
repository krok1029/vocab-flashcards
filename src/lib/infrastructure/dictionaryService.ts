// src/lib/infrastructure/dictionaryService.ts

import { fetchFromDictionaryApiDev } from './providers/dictionaryapi.dev/fetch';
import type { DictionaryEntry } from '$lib/domain/dictionary';

export async function fetchDictionaryEntry(word: string): Promise<DictionaryEntry | null> {
  const entry = await fetchFromDictionaryApiDev(word);
  if (entry) return entry;

  // 未來可支援 fallback
  // const entry2 = await fetchFromCambridgeAPI(word);
  // if (entry2) return entry2;

  return null;
}
