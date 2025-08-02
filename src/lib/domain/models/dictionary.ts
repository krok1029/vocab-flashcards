export type DictionaryEntry = {
  word: string;
  phonetic?: string;
  audio?: string;
  meanings: {
    partOfSpeech: string;
    definitions: {
      definition: string;
      example?: string;
      synonyms?: string[];
      antonyms?: string[];
    }[];
    synonyms?: string[];
    antonyms?: string[];
  }[];
};
