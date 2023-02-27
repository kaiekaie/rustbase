-- Your SQL goes here


CREATE  TYPE COLUMN_TYPES AS ENUM (
  'text',
  'number',
  'mail'
);

CREATE TABLE schema(
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  column_type COLUMN_TYPES,
  required BOOLEAN,
  uniques BOOLEAN
);