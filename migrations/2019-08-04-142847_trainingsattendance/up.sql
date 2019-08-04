-- Your SQL goes here
CREATE TABLE 'users' (
  'id' INTEGER NOT NULL PRIMARY KEY,
  'username' TEXT NOT NULL,
  'firstname' TEXT,
  'lastname' TEXT,
  'password' TEXT NOT NULL,
  'email' TEXT UNIQUE
);