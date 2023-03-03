CREATE TABLE games (
  id TEXT PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  user_a_id TEXT NOT NULL,
  user_b_id TEXT NOT NULL,
  goal REAL NOT NULL,
  current_value REAL NOT NULL,
  created_at TEXT NOT NULL
)
