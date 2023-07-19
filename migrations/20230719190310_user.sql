CREATE TABLE IF NOT EXISTS user (
	id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
	username VARCHAR(32) NOT NULL UNIQUE,
	email VARCHAR(32) NOT NULL UNIQUE,
	password CHAR(64) NOT NULL,
	wallet FLOAT NOT NULL DEFAULT 0.0,
	CONSTRAINT chk_username_length CHECK (LENGTH(username) >= 8 AND LENGTH(username) <= 32),
	CONSTRAINT chk_email_format CHECK (email REGEXP '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$')
);
