use actix_web::Scope;

pub mod admins;
pub mod collections;
pub mod users;
pub trait CreateScope {
    fn create_scope() -> Scope;
}
