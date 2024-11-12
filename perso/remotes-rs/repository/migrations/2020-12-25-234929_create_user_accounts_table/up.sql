CREATE TYPE user_account_status AS ENUM ('invalid', 'valid');
CREATE TABLE user_accounts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  user_id uuid NOT NULL REFERENCES users(id),
  status user_account_status NOT NULL,
  comment VARCHAR(280),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('user_accounts');
CREATE UNIQUE INDEX user_accounts_user_id ON user_accounts (user_id);
