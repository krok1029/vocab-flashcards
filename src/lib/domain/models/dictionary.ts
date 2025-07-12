export type DictionaryEntry = {
  word: string;
  phonetic?: string;
  audio?: string;
  meanings: {
    partOfSpeech: string;
    definitions: {
      definition: string;
      example?: string;
    }[];
  }[];
};
