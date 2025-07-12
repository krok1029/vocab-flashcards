// Domain types will be defined here
export interface WordCard {
  id?: number;
  word: string;
  definition: string;
  example?: string;
  created_at?: string;
}

export * from './wordCard';
