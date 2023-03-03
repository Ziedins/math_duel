CREATE TABLE moves (
  id TEXT PRIMARY KEY NOT NULL,
  game_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  value VARCHAR NOT NULL,
  created_at TEXT NOT NULL
)
