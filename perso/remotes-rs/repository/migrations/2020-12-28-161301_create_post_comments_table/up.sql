CREATE TABLE post_comments (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  post_id uuid REFERENCES posts(id) NOT NULL,
  user_id uuid REFERENCES users(id) NOT NULL,
  comment TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('post_comments');
CREATE INDEX post_comments_post_id ON post_comments (post_id);
CREATE INDEX post_comments_user_id ON post_comments (user_id);
