CREATE TABLE users (
	id            VARCHAR     NOT NULL  PRIMARY KEY,
	created_at    TIMESTAMPTZ NOT NULL  DEFAULT NOW(),
	username      VARCHAR     NOT NULL  UNIQUE,
    password      VARCHAR     NOT NULL,
	password_salt VARCHAR     NOT NULL,
	experience    FLOAT       NOT NULL  DEFAULT 0,
	level         SMALLINT    NOT NULL  DEFAULT 1
);

CREATE TABLE tokens (
	token         VARCHAR     NOT NULL  PRIMARY KEY,
	owner         VARCHAR     NOT NULL  UNIQUE       REFERENCES   users(id),
	created_at    TIMESTAMPTZ NOT NULL  DEFAULT NOW()
);

CREATE TABLE tiles (
	id            VARCHAR NOT NULL PRIMARY KEY,
	owner         VARCHAR NOT NULL REFERENCES users(id),
	name          VARCHAR NOT NULL,
	completion    BYTEA,
	type          SMALLINT NOT NULL
)