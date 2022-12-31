-- Your SQL goes here
CREATE TABLE user_friends (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  friend_id TEXT NOT NULL,
  friend_name TEXT NOT NULL,
  created_at TEXT NOT NULL
)