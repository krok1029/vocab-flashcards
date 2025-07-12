-- Your SQL goes here
CREATE TABLE word_cards (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  word TEXT NOT NULL UNIQUE,
  pos TEXT,
  definition TEXT,
  pronunciation TEXT,
  verbs TEXT,
  familiarity INTEGER DEFAULT 0,
  seen_count INTEGER DEFAULT 1,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
