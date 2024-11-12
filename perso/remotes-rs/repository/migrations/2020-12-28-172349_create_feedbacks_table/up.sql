CREATE TABLE feedbacks (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  company_id uuid REFERENCES companies(id) NOT NULL,
  user_id uuid REFERENCES users(id) NOT NULL,
  feedback TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT
  diesel_manage_updated_at('feedbacks');
CREATE INDEX feedbacks_company_id_user_id ON feedbacks (company_id, user_id);
