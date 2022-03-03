CREATE TABLE users (
	id         VARCHAR     NOT NULL  PRIMARY KEY,
	created_at TIMESTAMPTZ NOT NULL  DEFAULT NOW(),
	username   VARCHAR     NOT NULL,
	password   VARCHAR     NOT NULL,
	salt       VARCHAR     NOT NULL,
	experience FLOAT                 DEFAULT 0,
	level      SMALLINT              DEFAULT 1
);

