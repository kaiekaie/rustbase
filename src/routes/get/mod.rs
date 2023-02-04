#[get("/<name>/<age>")]
pub fn get(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
