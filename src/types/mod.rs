pub mod request;
pub mod config;
pub mod responder;
pub mod entry;
#[database("api-db")]
pub struct DBPool(diesel::PgConnection);