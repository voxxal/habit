CREATE TABLE users (
	id VARCHAR NOT NULL PRIMARY KEY,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	experience FLOAT DEFAULT 0,
	level SMALLINT DEFAULT 1
);

