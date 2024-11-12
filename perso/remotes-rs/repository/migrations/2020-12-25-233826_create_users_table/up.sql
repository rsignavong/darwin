CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  email VARCHAR(254) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('users');
CREATE UNIQUE INDEX users_email ON users (email);
