CREATE TABLE user
(
    username VARCHAR(20)  NOT NULL PRIMARY KEY,
    nickname VARCHAR(20)  NOT NULL,
    password VARCHAR(255) NOT NULL,
    email    VARCHAR(255) NOT NULL,
    avatar   VARCHAR(255) DEFAULT NULL
);
