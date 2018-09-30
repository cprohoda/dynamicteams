-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY key,
  email TEXT NOT NULL,
  skills TEXT[],
  tasks TEXT[]
)
