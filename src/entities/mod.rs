use diesel::prelude::*;
use rustbase::{
    establish_connection,
    models::{Documents, DocumentsWithSchemas, Schemas},
    schema::{documents, documents_to_schemas, schemas},
};
use std::error::Error;

pub fn getDocumentsWithSchemas() -> Result<(), Box<dyn Error + Send + Sync>> {
    let connection = &mut establish_connection();

    let documents = documents::table
        .select(Documents::as_select())
        .load(connection)?;

    let schemas = schemas::table
        .select(Schemas::as_select())
        .load(connection)?;

    let documentsAndSchema = DocumentsWithSchemas::belonging_to(&documents)
        .inner_join(schemas::table)
        .select(Schemas::as_select())
        .load(connection)?;

    let results = documentsAndSchema
        .grouped_by(&documents)
        .into_iter()
        .zip(documents)
        .map(|(b, documents)| (documents, b.into_iter().map(|(_, schema)| schema).collect()))
        .collect();

    Ok(())
}
