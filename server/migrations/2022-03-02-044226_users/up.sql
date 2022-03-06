CREATE TABLE users (
	id            VARCHAR     NOT NULL  PRIMARY KEY,
	created_at    TIMESTAMPTZ NOT NULL  DEFAULT NOW(),
	username      VARCHAR     NOT NULL,
        password      VARCHAR     NOT NULL,
	password_salt VARCHAR     NOT NULL,
	experience    FLOAT       NOT NULL  DEFAULT 0,
	level         SMALLINT    NOT NULL  DEFAULT 1
);

