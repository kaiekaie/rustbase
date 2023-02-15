use crate::entities::collectionsLoad::postsLoad;

#[get("/collections")]
pub fn get() -> String {
    let vals = postsLoad();

    if (!vals.is_empty()) {
        format!(
            "getting collections {}",
            serde_json::to_string(&vals).unwrap()
        )
    } else {
        format!("empty collections")
    }
}
