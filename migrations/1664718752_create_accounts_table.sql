CREATE TABLE accounts(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);
