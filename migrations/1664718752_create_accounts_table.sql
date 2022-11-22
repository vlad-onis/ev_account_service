CREATE TABLE accounts(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

INSERT INTO accounts (id, email, username, password) VALUES ('ffdaed86-c347-44b4-a0bb-7b61730562e3', 'vladonis@gmail.com', 'vladonis', 'test1234');
