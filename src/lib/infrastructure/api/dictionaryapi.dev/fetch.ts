// src/lib/infrastructure/dictionaryapi.dev/fetch.ts

import type { DictionaryEntry } from '$lib/domain/models';
import { adaptDictionaryApiResponse } from './dictionaryApiAdapter';
import type { RawDictionaryEntry } from './rawTypes';

export async function fetchFromDictionaryApiDev(
  word: string
): Promise<DictionaryEntry | null> {
  const res = await fetch(
    `https://api.dictionaryapi.dev/api/v2/entries/en/${word}`
  );
  const json = await res.json();

  if (!Array.isArray(json)) return null;

  return adaptDictionaryApiResponse(json as RawDictionaryEntry[]);
}
