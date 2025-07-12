import { toast } from 'svelte-sonner';
import { dictionaryStore } from '$lib/application/stores/dictionaryStore';
import { searchWord } from '$lib/application/services/searchDictionary';
import { WordCardService } from '$lib/application/services/wordCardService';

export class SearchCommand {
  static async execute(query: string): Promise<void> {
    if (!query.trim()) return;

    const word = query.trim();
    
    dictionaryStore.setLoading(true);
    dictionaryStore.clearError();
    dictionaryStore.setEntry(null);
    dictionaryStore.setExistsInCard(false);

    try {
      // First check if word exists in local database
      const existingCard = await WordCardService.getWordCard(word);

      if (existingCard) {
        const entry = WordCardService.mapWordCardToDictionaryEntry(existingCard);
        dictionaryStore.setEntry(entry);
        dictionaryStore.setExistsInCard(true);
      } else {
        // Search from external dictionary API
        const entry = await searchWord(word);
        dictionaryStore.setEntry(entry);
        dictionaryStore.setExistsInCard(false);
      }
    } catch (error) {
      const errorMessage = `Search failed: ${error}`;
      dictionaryStore.setError(errorMessage);
      toast.error(errorMessage);
    } finally {
      dictionaryStore.setLoading(false);
    }
  }
}

export class SaveWordCardCommand {
  static async execute(): Promise<void> {
    const state = dictionaryStore;
    let currentEntry: any = null;
    
    // Get current entry from store
    const unsubscribe = state.subscribe(value => {
      currentEntry = value.entry;
    });
    unsubscribe();

    if (!currentEntry) {
      toast.error('No word to save');
      return;
    }

    try {
      await WordCardService.saveWordCard(currentEntry);
      toast.success(`已儲存：${currentEntry.word}`);
      dictionaryStore.setExistsInCard(true);
    } catch (error) {
      const errorMessage = `儲存失敗: ${error}`;
      toast.error(errorMessage);
    }
  }
}
