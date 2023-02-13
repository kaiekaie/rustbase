-- This file should undo anything in `up.sql`
DROP TABLE collections;
DROP TRIGGER IF EXISTS update_customer_modtime ON collections;
DROP FUNCTION IF EXISTS update_modified_column;