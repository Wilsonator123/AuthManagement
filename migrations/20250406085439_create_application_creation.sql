-- Add migration script here
CREATE TABLE app(
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT
)