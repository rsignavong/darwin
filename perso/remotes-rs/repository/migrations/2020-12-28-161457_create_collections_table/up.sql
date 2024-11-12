CREATE TABLE collections (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name VARCHAR(80) NOT NULL,
  rules jsonb NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('collections');
