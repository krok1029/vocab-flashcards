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

  static async getAllWordCards(): Promise<WordCard[]> {
    try {
      return await invoke<WordCard[]>('get_all_word_cards');
    } catch (error) {
      console.error('Failed to get all word cards:', error);
      throw new Error(`Failed to get all word cards: ${error}`);
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

  static async updateFamiliarity(cardId: number, familiarity: number): Promise<void> {
    try {
      await invoke('update_word_card_familiarity', {
        cardId,
        familiarityLevel: familiarity,
      });
    } catch (error) {
      console.error('Failed to update familiarity:', error);
      throw new Error(`Failed to update familiarity: ${error}`);
    }
  }

  static async deleteWordCard(cardId: number): Promise<void> {
    try {
      await invoke('delete_word_card', { cardId });
    } catch (error) {
      console.error('Failed to delete word card:', error);
      throw new Error(`Failed to delete word card: ${error}`);
    }
  }

  static async incrementSeenCount(cardId: number): Promise<void> {
    try {
      await invoke('increment_word_card_seen_count', { cardId });
    } catch (error) {
      console.error('Failed to increment seen count:', error);
      throw new Error(`Failed to increment seen count: ${error}`);
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
