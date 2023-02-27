-- Your SQL goes here



CREATE TABLE document_to_schema (

  document_id INTEGER REFERENCES document(id),
  schema_id  INTEGER REFERENCES schema(id),

  PRIMARY KEY(document_id, schema_id)
);
