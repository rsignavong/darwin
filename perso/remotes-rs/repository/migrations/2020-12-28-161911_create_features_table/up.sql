CREATE TABLE features (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name VARCHAR(80) NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('features');
