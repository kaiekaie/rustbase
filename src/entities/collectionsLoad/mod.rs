use diesel::prelude::*;
use models::*;
use rustbase::*;

pub fn collectionsLoad() {
    use self::schema::collections::dsl::*;

    let connection = &mut establish_connection();
    let results = collections
        .limit(5)
        .load::<Collections>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {}
}
