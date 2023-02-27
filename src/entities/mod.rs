use diesel::prelude::*;

use rustplatform::{
    establish_connection,
    models::{Document, DocumentWithschema, Schema},
    schema::{document, document_to_schema, schema},
};
use std::error::Error;

pub fn get_document_with_schema() -> Result<Vec<Document>, Box<dyn Error + Send + Sync>> {
    let connection = &mut establish_connection();

    let all_document = document::table
        .select(Document::as_select())
        .load(connection)?;

    let documents = DocumentWithschema::belonging_to(&all_document)
        .inner_join(schema::table)
        .select(Schema::as_select())
        .load(connection)?;

    /*     let docs_schemas = DocumentWithschema::belonging_to(&documents)
    .inner_join(schema::table)
    .select((Document::as_select(), Schema::as_select()))
    .load(connection)?; */

    Ok(all_document)
}
