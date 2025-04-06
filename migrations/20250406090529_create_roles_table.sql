-- Add migration script here
CREATE TABLE role (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  app_id UUID NOT NULL,
  FOREIGN KEY (app_id) REFERENCES app(id)
)