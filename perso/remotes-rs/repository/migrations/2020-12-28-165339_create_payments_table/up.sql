CREATE TABLE payments (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  purchase_order_id uuid REFERENCES purchase_orders(id) NOT NULL,
  metadata JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('payments');
CREATE UNIQUE INDEX payments_purchase_order_id ON payments (purchase_order_id);
