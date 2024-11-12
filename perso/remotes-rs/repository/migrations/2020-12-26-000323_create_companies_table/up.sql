CREATE TABLE companies (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name VARCHAR(80) NOT NULL,
  description TEXT NOT NULL,
  tag_line VARCHAR(80),
  logo_url VARCHAR(2048),
  website_url VARCHAR(2048),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('companies');
ALTER TABLE
  users
ADD
  COLUMN company_id uuid REFERENCES companies(id);
CREATE INDEX users_company_id ON users (company_id);
