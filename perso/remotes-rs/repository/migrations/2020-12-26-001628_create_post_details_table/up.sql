CREATE TYPE post_detail_status AS ENUM ('draft', 'reviewed');
CREATE TABLE post_details (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  post_id uuid NOT NULL REFERENCES posts(id),
  user_id uuid NOT NULL REFERENCES users(id),
  details jsonb NOT NULL,
  version SMALLINT DEFAULT 1 NOT NULL,
  status post_detail_status NOT NULL DEFAULT 'draft',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('post_details');
CREATE INDEX post_details_post_id ON post_details (post_id);
CREATE INDEX post_details_user_id ON post_details (user_id);
