CREATE TABLE moves (
    id TEXT PRIMARY KEY NOT NULL,
    game_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    operator VARCHAR NOT NULL,
    term VARCHAR NOT NULL,
    created_at TEXT NOT NULL
)
