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

    let everyThing = schemas
        .grouped_by(&all_documents)
        .into_iter()
        .zip(all_documents)
        .map(|(schema, doc)| {
            let document_with_schema = DocumentWithSchemas::new(&doc, schema);

            return document_with_schema;
        })
        .collect::<Vec<DocumentWithSchemas>>();

    /*     let documents = document::table
    .inner_join(schema::table)
    .select((Document::as_select(), Schema::as_select()))
    .load::<(Document, Schema)>(connection)?
    .grouped_by(Schema::as_select); */

    Ok(everyThing)
}
