CREATE TABLE post_collections (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  approved_post_id uuid REFERENCES approved_posts(id) NOT NULL,
  collection_id uuid REFERENCES collections(id) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('post_collections');
CREATE UNIQUE INDEX post_collections_approved_post_id_collection_id ON post_collections (approved_post_id, collection_id);
