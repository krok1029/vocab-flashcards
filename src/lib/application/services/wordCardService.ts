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
    
    // 解析存儲的詞性
    let partOfSpeechList: string[] = [];
    try {
      partOfSpeechList = JSON.parse(card.pos || '[]');
    } catch {
      partOfSpeechList = card.pos ? [card.pos] : [];
    }

    // 解析定義文本，重建 meanings 結構
    const definitionText = card.definition || '';
    const definitionBlocks = definitionText.split('\n\n');
    
    const meanings = partOfSpeechList.map((pos, index) => {
      // 找到對應此詞性的定義
      const relevantDefinitions = definitionBlocks.filter(block => 
        block.includes(`[${pos}]`)
      );
      
      const definitions = relevantDefinitions.length > 0 
        ? relevantDefinitions.map(block => {
            // 提取定義文本（移除詞性標記）
            const lines = block.split('\n');
            const definitionLine = lines[0].replace(`[${pos}] `, '');
            
            // 提取例句
            const exampleLine = lines.find(line => line.startsWith('例句: '));
            const example = exampleLine ? exampleLine.replace('例句: ', '') : undefined;
            
            return {
              definition: definitionLine,
              example,
            };
          })
        : [
            {
              definition: definitionText || '',
              example: undefined,
            },
          ];

      return {
        partOfSpeech: pos,
        definitions,
      };
    });

    return {
      word: card.word,
      phonetic: pronunciation?.phonetic ?? '',
      audio: pronunciation?.audio ?? '',
      meanings: meanings.length > 0 ? meanings : [
        {
          partOfSpeech: partOfSpeechList[0] || '',
          definitions: [
            {
              definition: definitionText || '',
              example: undefined,
            },
          ],
        },
      ],
    };
  }

  private static prepareCardPayload(entry: DictionaryEntry): WordCardPayload {
    // 收集所有定義，按詞性分組
    const allDefinitions = entry.meanings.flatMap(meaning => 
      meaning.definitions.map((def, index) => {
        let definitionText = `[${meaning.partOfSpeech}] ${def.definition}`;
        
        // 加入例句
        if (def.example) {
          definitionText += `\n例句: ${def.example}`;
        }
        
        // 加入同義詞（優先使用定義層級的，再使用詞性層級的）
        const synonyms = def.synonyms || meaning.synonyms;
        if (synonyms && synonyms.length > 0) {
          definitionText += `\n同義詞: ${synonyms.join(', ')}`;
        }
        
        // 加入反義詞（優先使用定義層級的，再使用詞性層級的）
        const antonyms = def.antonyms || meaning.antonyms;
        if (antonyms && antonyms.length > 0) {
          definitionText += `\n反義詞: ${antonyms.join(', ')}`;
        }
        
        return definitionText;
      })
    );

    return {
      word: entry.word,
      pos: JSON.stringify(entry.meanings.map((m) => m.partOfSpeech)),
      definition: allDefinitions.join('\n\n'), // 用雙換行分隔不同定義
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
