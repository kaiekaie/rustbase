-- This file should undo anything in `up.sql`
DROP TABLE documents;
DROP TRIGGER IF EXISTS update_customer_modtime ON documents;
DROP FUNCTION IF EXISTS update_modified_column;