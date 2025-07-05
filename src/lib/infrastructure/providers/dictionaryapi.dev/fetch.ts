// src/lib/infrastructure/dictionaryapi.dev/fetch.ts

import { adaptDictionaryApiResponse } from '$lib/infrastructure/providers/dictionaryapi.dev/dictionaryApiAdapter';
import type { RawDictionaryEntry } from '$lib/infrastructure/providers/dictionaryapi.dev/rawTypes';
import type { DictionaryEntry } from '$lib/domain/dictionary';


export async function fetchFromDictionaryApiDev(word: string): Promise<DictionaryEntry | null> {
  const res = await fetch(`https://api.dictionaryapi.dev/api/v2/entries/en/${word}`);
  const json = await res.json();

  if (!Array.isArray(json)) return null;

  return adaptDictionaryApiResponse(json as RawDictionaryEntry[]);
}
