-- Your SQL goes here



CREATE TABLE documents_to_schemas (

  document_id SERIAL REFERENCES documents(id),
  schema_id  SERIAL REFERENCES schemas(id),

  PRIMARY KEY(document_id, schema_id)
);
