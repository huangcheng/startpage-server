CREATE TABLE category (
    id VARCHAR(36) NOT NULL DEFAULT (uuid()) PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description VARCHAR(255) NOT NULL
);
