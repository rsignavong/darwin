CREATE TABLE promotions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  product_id uuid REFERENCES products(id) NOT NULL,
  name VARCHAR(80) NOT NULL,
  description TEXT NOT NULL,
  begin_date TIMESTAMPTZ NOT NULL,
  end_date TIMESTAMPTZ NOT NULL,
  price INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('promotions');
CREATE INDEX promotions_product_id ON promotions (product_id);
