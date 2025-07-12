export interface WordCard {
  id?: number;
  word: string;
  pos?: string;
  definition: string;
  pronunciation?: string;
  verbs?: string;
  familiarity: number;
  seen_count: number;
  created_at?: string;
  updated_at?: string;
}

export interface WordCardPayload {
  word: string;
  pos: string;
  definition: string;
  pronunciation: string;
  verbs: string;
  familiarity: number;
  seen_count: number;
}
