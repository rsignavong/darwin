CREATE TABLE approved_posts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  post_id uuid REFERENCES posts(id) NOT NULL,
  post_detail_id uuid REFERENCES post_details(id) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('approved_posts');
CREATE UNIQUE INDEX approved_posts_post_id ON approved_posts (post_id);
CREATE UNIQUE INDEX approved_posts_post_detail_id ON approved_posts (post_detail_id);
