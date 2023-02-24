-- This file should undo anything in `up.sql`


DROP TABLE documents_to_schema;
DROP TABLE schemas;
DROP TABLE documents;
DROP TYPE COLUMN_TYPES;
DROP TRIGGER IF EXISTS update_customer_modtime ON documents;
DROP FUNCTION IF EXISTS update_modified_column;