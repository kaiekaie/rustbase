use diesel::prelude::*;

use diesel::result::Error;
use rocket::{response::status::NotFound, serde::json::Json};

use rustplatform::{
    establish_connection,
    models::{Document, DocumentWithSchemas, Record, Schema},
    schema::{document, record},
};
use serde_json::{json, Value};

fn merge(a: &mut Value, b: Value) -> &mut Value {
    if let Value::Object(ref mut map1) = a {
        if let Value::Object(ref map2) = b {
            map1.extend(map2.iter().map(|(k, v)| (k.to_owned(), v.to_owned())));
        }
    }

    return a;
}

pub fn get_documents_with_schema() -> Result<Vec<DocumentWithSchemas>, Error> {
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

pub fn recordto_json(record: Record) -> Value {
    let mut data = serde_json::to_value(&record.data).unwrap();

    let records = serde_json::to_value(&record).unwrap();
    let mut newData = merge(&mut data, records).take();
    if let Value::Object(ref mut map) = newData {
        map.remove("data");
    }
    return newData;
}

pub fn get_records() -> Result<Vec<Record>, Error> {
    let connection = &mut establish_connection();
    let resp = record::table.load::<Record>(connection).unwrap();
    Ok(resp)
}
pub fn get_records_by_name(record_name: String) -> Result<Json<Value>, NotFound<String>> {
    let connection = &mut establish_connection();
    let resp: Result<Record, diesel::result::Error> = record::table
        .filter(record::name.eq(record_name))
        .first(connection);

    if let Some(record) = resp.ok() {
        Ok(Json(recordto_json(record)))
    } else {
        Err(NotFound(String::from("error")))
    }
}

pub fn get_document_with_schema_based_on_id(
    document_id: i32,
) -> Result<Vec<DocumentWithSchemas>, Error> {
    let connection = &mut establish_connection();

    let all_documents = document::table
        .filter(document::id.eq(document_id))
        .load::<Document>(connection)?;

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
