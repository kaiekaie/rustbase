-- Your SQL goes here


CREATE TABLE record (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT NOW(),
  modified TIMESTAMP NOT NULL DEFAULT NOW(),
  document_id INTEGER REFERENCES document(id),
  data json 
);

CREATE TRIGGER update_record_modtime BEFORE UPDATE ON record FOR EACH ROW EXECUTE PROCEDURE update_modified_column();