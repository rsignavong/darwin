CREATE TABLE purchase_orders (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  company_id uuid REFERENCES companies(id) NOT NULL,
  user_id uuid REFERENCES users(id) NOT NULL,
  purchase_order JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('purchase_orders');
CREATE INDEX purchase_orders_company_id_user_id ON purchase_orders (company_id, user_id);
