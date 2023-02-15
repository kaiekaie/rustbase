use self::models::*;
use diesel::prelude::*;
use rustbase::*;

pub fn posts_load() -> Vec<rustbase::models::RustbaseCollections> {
    use self::schema::documents::dsl::*;
    let connection = &mut establish_connection();
    return documents
        .limit(5)
        .load::<RustbaseCollections>(connection)
        .expect("Error loading posts");
}
