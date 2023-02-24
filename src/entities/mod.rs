

use diesel::prelude::*;
use rustbase::{
    establish_connection,
    models::{Documents, DocumentsWithSchemas, Schemas},
    schema::{documents, documents_to_schemas, schemas},
};
use std::error::Error;
use rustbase::schema::documents_to_schemas::dsl::documents_to_schemas;

pub fn get_documents_with_schemas() -> Result<Vec<Documents>, Box<dyn Error + Send + Sync>> {
    let connection = &mut establish_connection();

    let all_documents = documents::table
        .select(Documents::as_select())
        .load(connection)?;




    //
    // let results = documentsAndSchema
    //     .grouped_by(&documents)
    //     .into_iter()
    //     .zip(documents)
    //     .map(|(b, documents)| (documents, b.into_iter().map(|(_, schema)| schema).collect()))
    //     .collect();

    Ok(all_documents)
}
