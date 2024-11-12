CREATE TABLE tags (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  tag VARCHAR(40) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('tags');
CREATE UNIQUE INDEX tags_tag ON tags (tag);
CREATE TABLE tagged_posts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  post_id uuid REFERENCES posts(id) NOT NULL,
  tag_id uuid REFERENCES tags(id) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('tagged_posts');
CREATE UNIQUE INDEX tagged_posts_post_id_tag_id ON tagged_posts (post_id, tag_id);
