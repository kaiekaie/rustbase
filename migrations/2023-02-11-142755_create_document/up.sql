-- Your SQL goes here

CREATE TABLE document (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT NOW(),
  modified TIMESTAMP NOT NULL DEFAULT NOW(),
  listRule VARCHAR,
  viewRule VARCHAR,
  createRule VARCHAR,
  updateRule VARCHAR,
  deleteRule VARCHAR
);


CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.modified = now();
    RETURN NEW;   
END;
$$ language 'plpgsql';

CREATE TRIGGER update_document_modtime BEFORE UPDATE ON document FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
