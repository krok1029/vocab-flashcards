import type { RawDictionaryEntry } from '$lib/infrastructure/providers/dictionaryapi.dev/rawTypes';
import type { DictionaryEntry } from '$lib/domain/dictionary';

export function adaptDictionaryApiResponse(raw: RawDictionaryEntry[]): DictionaryEntry | null {
  if (!raw || raw.length === 0) return null;

  const entry = raw[0]; // 只取第一筆最常用的解釋

  const audio = entry.phonetics?.find(p => p.audio)?.audio;

  return {
    word: entry.word,
    phonetic: entry.phonetic,
    audio,
    meanings: entry.meanings.map((m: any) => ({
      partOfSpeech: m.partOfSpeech,
      definitions: m.definitions.map((d: any) => ({
        definition: d.definition,
        example: d.example,
      })),
    })),
  };
}
