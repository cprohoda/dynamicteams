-- Your SQL goes here
CREATE TABLE orgs (
  id SERIAL PRIMARY key,
  code TEXT NOT NULL UNIQUE,
  name TEXT,
  employees INTEGER ARRAY
)
