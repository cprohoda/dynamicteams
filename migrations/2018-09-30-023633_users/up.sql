-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY key,
  email TEXT NOT NULL UNIQUE,
  skills TEXT ARRAY,
  tasks TEXT ARRAY
)
