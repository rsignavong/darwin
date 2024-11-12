CREATE TABLE credits (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  purchase_order_id uuid REFERENCES purchase_orders(id) NOT NULL,
  feature_id uuid REFERENCES features(id) NOT NULL,
  post_id uuid REFERENCES posts(id) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('credits');
CREATE UNIQUE INDEX credits_purchase_order_id_feature_id_post_id ON credits (purchase_order_id, feature_id, post_id);
