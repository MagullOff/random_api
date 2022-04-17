use rocket::get;
use rocket::response::status;
use crate::services::request as requestService;
use crate::services::db as dbService;
use crate::types::DBPool;
use crate::types::responder::ImageResponder;

#[get("/")]
pub fn get_random_image(conn: DBPool) -> Result<ImageResponder, status::BadRequest<&'static str>> {
    let mut url = "".to_string();
    let res = requestService::get_photo_url()
        .and_then(|s| {url = s.clone(); requestService::get_image_from_url(s)})
        .map(|v| ImageResponder::new(v))
        .map_err(|e| status::BadRequest(Some(e)));

    match res {
        Err(e) => Err(e),
        Ok(o) => {
            let db_res = dbService::insert_entry(url, conn);
            match db_res {
                Ok(_) => Ok(o),
                Err(x) => Err(status::BadRequest(Some(x)))
            }
        }
    }

}