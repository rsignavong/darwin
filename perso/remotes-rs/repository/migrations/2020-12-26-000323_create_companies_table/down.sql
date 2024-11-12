DROP INDEX users_company_id;
ALTER TABLE
  users DROP COLUMN company_id;
DROP TABLE companies;
