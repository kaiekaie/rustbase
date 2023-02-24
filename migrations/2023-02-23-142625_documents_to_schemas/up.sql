-- Your SQL goes here



CREATE TABLE documents_to_schemas (

  document_id INTEGER REFERENCES documents(id),
  schema_id  INTEGER REFERENCES schemas(id),

  PRIMARY KEY(document_id, schema_id)
);
