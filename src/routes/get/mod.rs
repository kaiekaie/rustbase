use crate::entities::collectionsLoad::posts_load;

#[get("/collections")]
pub fn get() -> String {
    let vals = posts_load();

    match !vals.is_empty() {
        true => serde_json::to_string(&vals).unwrap(),
        false => format!("empty collections"),
    }
}
