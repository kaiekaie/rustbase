-- Your SQL goes here

CREATE TABLE collections (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created timestamp default CURRENT_TIMESTAMP not null,
  modified  timestamp default CURRENT_TIMESTAMP not null
);

CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.modified = now();
    RETURN NEW;   
END;
$$ language 'plpgsql';

CREATE TRIGGER update_customer_modtime BEFORE UPDATE ON collections FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
