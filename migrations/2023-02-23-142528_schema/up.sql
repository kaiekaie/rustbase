-- Your SQL goes here


CREATE  TYPE COLUMN_TYPES AS ENUM (
  'text',
  'number',
  'mail',
  'relation'
);

CREATE TABLE schema(
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  column_type COLUMN_TYPES,
  required BOOLEAN,
  uniques BOOLEAN,
  document_id  INTEGER REFERENCES document(id)
);