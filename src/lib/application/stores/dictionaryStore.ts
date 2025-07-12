import { writable, derived } from 'svelte/store';
import type { DictionaryEntry } from '$lib/domain/models/dictionary';

export interface DictionaryState {
  query: string;
  entry: DictionaryEntry | null;
  loading: boolean;
  error: string;
  existsInCard: boolean;
}

const initialState: DictionaryState = {
  query: '',
  entry: null,
  loading: false,
  error: '',
  existsInCard: false,
};

function createDictionaryStore() {
  const { subscribe, set, update } = writable<DictionaryState>(initialState);

  return {
    subscribe,
    setQuery: (query: string) => update(state => ({ ...state, query })),
    setLoading: (loading: boolean) => update(state => ({ ...state, loading })),
    setError: (error: string) => update(state => ({ ...state, error })),
    setEntry: (entry: DictionaryEntry | null) => update(state => ({ ...state, entry })),
    setExistsInCard: (exists: boolean) => update(state => ({ ...state, existsInCard: exists })),
    reset: () => set(initialState),
    clearError: () => update(state => ({ ...state, error: '' })),
  };
}

export const dictionaryStore = createDictionaryStore();

// Derived stores for computed values
export const hasEntry = derived(dictionaryStore, $store => $store.entry !== null);
export const canSaveCard = derived(dictionaryStore, $store => $store.entry !== null && !$store.existsInCard);
