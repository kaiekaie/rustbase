use rocket::form::Form;

use core::result::Result;
#[derive(FromForm)]
pub struct Task<'r> {
    #[field(validate = len(1..))]
    description: &'r str,
    completed: bool,
}

#[post("/", data = "<task>")]
pub fn post(task: Form<Task<'_>>) -> Result<String, ()> {
    Ok(format!(
        "Hello, {} year old named {}!",
        task.description, task.completed
    ))
}
