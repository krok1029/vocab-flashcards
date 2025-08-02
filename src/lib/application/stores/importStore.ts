import { writable, derived } from 'svelte/store';
import type { ImportResult } from '$lib/application/commands/importCommand';

interface ImportState {
  isImporting: boolean;
  totalWords: number;
  currentIndex: number;
  currentWord: string;
  successCount: number;
  failedCount: number;
  existsCount: number;
  results: ImportResult[];
  cancelled: boolean;
}

const initialState: ImportState = {
  isImporting: false,
  totalWords: 0,
  currentIndex: 0,
  currentWord: '',
  successCount: 0,
  failedCount: 0,
  existsCount: 0,
  results: [],
  cancelled: false
};

function createImportStore() {
  const { subscribe, set, update } = writable<ImportState>(initialState);

  return {
    subscribe,
    
    startImport: (totalWords: number) => {
      set({
        ...initialState,
        isImporting: true,
        totalWords,
      });
    },

    setCurrentWord: (word: string, index: number) => {
      update(state => ({
        ...state,
        currentWord: word,
        currentIndex: index
      }));
    },

    incrementSuccess: () => {
      update(state => ({
        ...state,
        successCount: state.successCount + 1
      }));
    },

    incrementFailed: () => {
      update(state => ({
        ...state,
        failedCount: state.failedCount + 1
      }));
    },

    incrementExists: () => {
      update(state => ({
        ...state,
        existsCount: state.existsCount + 1
      }));
    },

    completeImport: (results: ImportResult[]) => {
      update(state => ({
        ...state,
        isImporting: false,
        results,
        currentWord: '',
        currentIndex: 0
      }));
    },

    cancelImport: () => {
      update(state => ({
        ...state,
        isImporting: false,
        cancelled: true,
        currentWord: '',
        currentIndex: 0
      }));
    },

    reset: () => {
      set(initialState);
    },

    getStats: () => {
      let stats = { successCount: 0, failedCount: 0, existsCount: 0 };
      const unsubscribe = subscribe(state => {
        stats = {
          successCount: state.successCount,
          failedCount: state.failedCount,
          existsCount: state.existsCount
        };
      });
      unsubscribe();
      return stats;
    }
  };
}

export const importStore = createImportStore();

// Derived stores for computed values
export const importProgress = derived(
  importStore,
  $store => $store.totalWords > 0 ? ($store.currentIndex / $store.totalWords) * 100 : 0
);

export const isImporting = derived(
  importStore,
  $store => $store.isImporting
);

export const hasResults = derived(
  importStore,
  $store => $store.results.length > 0
);
