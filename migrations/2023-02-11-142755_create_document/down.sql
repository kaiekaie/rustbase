-- This file should undo anything in `up.sql`


DROP TABLE document_to_schema;
DROP TABLE schema;
DROP TABLE document;
DROP TYPE COLUMN_TYPES;
DROP TRIGGER IF EXISTS update_customer_modtime ON document;
DROP FUNCTION IF EXISTS update_modified_column;