use rocket::get;
use rocket_contrib::json::Json;

use crate::{types::{DBPool, entry::ReturnEntry}, services::db::get_all_entries};

#[get("/")]
pub fn get_history(conn: DBPool) -> Result<Json<Vec<ReturnEntry>>,&'static str>{
    get_all_entries(conn)
        .and_then(|vec| Ok(Json(vec)))
}