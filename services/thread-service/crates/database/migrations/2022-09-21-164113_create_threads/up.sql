CREATE TABLE threads
(
    id    VARCHAR(255) DEFAULT (uuid()) NOT NULL,
    board VARCHAR(255)                  NOT NULL,
    user  VARCHAR(255)                  NOT NULL,
    title VARCHAR(255)                  NOT NULL,
    text  MEDIUMTEXT                    NULL,
    media MEDIUMTEXT                    NULL,
    score int          DEFAULT 0        NOT NULL,
    CONSTRAINT threads_pk
        PRIMARY KEY (id)
);