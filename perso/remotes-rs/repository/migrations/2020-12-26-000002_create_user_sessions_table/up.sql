CREATE TABLE user_sessions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  user_account_id uuid NOT NULL REFERENCES user_accounts(id),
  code INTEGER NOT NULL,
  token VARCHAR(26),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('user_sessions');
CREATE UNIQUE INDEX user_sessions_token ON user_sessions (token);
CREATE INDEX user_sessions_user_account_id ON user_sessions (user_account_id);
