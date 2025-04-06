-- Add migration script here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  fname TEXT NOT NULL,
  lname TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  app_id UUID NOT NULL,
  FOREIGN KEY(app_id) REFERENCES app(id)
);