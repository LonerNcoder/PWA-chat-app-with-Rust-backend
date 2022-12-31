-- Your SQL goes here
CREATE TABLE conversations (
  id TEXT PRIMARY KEY NOT NULL,
  room_id TEXT NOT NULL,
  sender_id TEXT NOT NULL,
  receiver_id TEXT NOT NULL,
  content VARCHAR NOT NULL,
  seen BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TEXT NOT NULL
)