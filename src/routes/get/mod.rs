#[get("/collections")]
pub fn get() -> String {
    load();
    format!("getting collections")
}
