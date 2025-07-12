import { invoke } from '@tauri-apps/api/core';
import type { DictionaryEntry } from '$lib/domain/models/dictionary';
import type { WordCard, WordCardPayload } from '$lib/domain/types/wordCard';
import { parsePronunciation, stringifyPronunciation } from '$lib/presentation/utils';

export class WordCardService {
  static async getWordCard(word: string): Promise<WordCard | null> {
    try {
      return await invoke<WordCard | null>('get_word_card_by_word', {
        wordQuery: word,
      });
    } catch (error) {
      console.error('Failed to get word card:', error);
      throw new Error(`Failed to get word card: ${error}`);
    }
  }

  static async saveWordCard(entry: DictionaryEntry): Promise<void> {
    const payload = this.prepareCardPayload(entry);
    
    try {
      await invoke('save_word_card', { card: payload });
    } catch (error) {
      console.error('Failed to save word card:', error);
      throw new Error(`Failed to save word card: ${error}`);
    }
  }

  static mapWordCardToDictionaryEntry(card: WordCard): DictionaryEntry {
    const pronunciation = parsePronunciation(card.pronunciation);
    
    return {
      word: card.word,
      phonetic: pronunciation?.phonetic ?? '',
      audio: pronunciation?.audio ?? '',
      meanings: card.pos
        ? [
            {
              partOfSpeech: card.pos,
              definitions: [
                {
                  definition: card.definition ?? '',
                  example: undefined, // DB doesn't store examples yet
                },
              ],
            },
          ]
        : [],
    };
  }

  private static prepareCardPayload(entry: DictionaryEntry): WordCardPayload {
    return {
      word: entry.word,
      pos: JSON.stringify(entry.meanings.map((m) => m.partOfSpeech)),
      definition: entry.meanings[0]?.definitions?.[0]?.definition || '',
      pronunciation: stringifyPronunciation({
        phonetic: entry.phonetic || '',
        audio: entry.audio || '',
      }),
      verbs: JSON.stringify({}), // Future expansion
      familiarity: 0,
      seen_count: 1,
    };
  }
}
