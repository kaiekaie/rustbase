use diesel::prelude::*;

use rustplatform::{
    establish_connection,
    models::{Document, DocumentWithSchemas, Schema},
    schema::{
        document::{self, id},
        schema,
    },
};

use std::error::Error;

pub fn get_document_with_schema() -> Result<Vec<DocumentWithSchemas>, Box<dyn Error + Send + Sync>>
{
    let connection = &mut establish_connection();
    let all_documents = document::table.load::<Document>(connection)?;
    let schemas: Vec<Schema> = Schema::belonging_to(&all_documents)
        .select(Schema::as_select())
        .load(connection)?;

    let documents = schemas
        .grouped_by(&all_documents)
        .into_iter()
        .zip(all_documents)
        .map(|(schema, doc)| DocumentWithSchemas::new(&doc, schema))
        .collect::<Vec<DocumentWithSchemas>>();
    Ok(documents)
}
