CREATE TABLE packagings (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  product_id uuid REFERENCES products(id) NOT NULL,
  feature_id uuid REFERENCES features(id) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('packagings');
CREATE UNIQUE INDEX packagings_product_id_feature_id ON packagings (product_id, feature_id);
