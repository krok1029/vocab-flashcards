// src/lib/infrastructure/dictionaryApi.ts

import type { DictionaryEntry } from '$lib/domain/dictionary';

export type FetchDictionaryResult =
  | { success: true; entries: DictionaryEntry[] }
  | { success: false; reason: 'not_found' | 'network_error'; message: string };

export async function fetchDictionaryEntry(word: string): Promise<FetchDictionaryResult> {
  try {
    const res = await fetch(`https://api.dictionaryapi.dev/api/v2/entries/en/${word}`);
    const data = await res.json();

    if (Array.isArray(data)) {
      return { success: true, entries: data };
    }

    if ('title' in data && data.title === 'No Definitions Found') {
      return {
        success: false,
        reason: 'not_found',
        message: data.message ?? 'No definition found',
      };
    }

    return {
      success: false,
      reason: 'network_error',
      message: 'Unexpected response format',
    };
  } catch (err) {
    return {
      success: false,
      reason: 'network_error',
      message: (err as Error).message,
    };
  }
}
