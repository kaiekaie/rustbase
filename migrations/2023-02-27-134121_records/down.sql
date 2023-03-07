-- This file should undo anything in `up.sql`

DROP TABLE record;
DROP TRIGGER IF EXISTS update_record_modtime ON document;