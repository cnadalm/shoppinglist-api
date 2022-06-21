-- Add up migration script here
CREATE TABLE "shopping_list_item"(
	id TEXT PRIMARY KEY NOT NULL,
	"name" TEXT NOT NULL,
	quantity TEXT NOT NULL,
	created_at TEXT NOT NULL
);
