-- Add up migration script here
CREATE TABLE IF NOT EXISTS "shopping_list_item"(
	id TEXT PRIMARY KEY NOT NULL,
	"name" TEXT NOT NULL,
	quantity TEXT NOT NULL,
	"image" TEXT,
	"state" TEXT NOT NULL DEFAULT 'PENDING',
	created_at TEXT NOT NULL,
	completed_at TEXT
);