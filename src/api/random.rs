use rocket::get;
use rocket::response::status;
use crate::services::request as RequestService;
use crate::services::db as dbService;
use crate::types::DBPool;
use crate::types::responder::ImageResponder;

//decided to return a binary of the image 
//so that it would be displayed in a browser 
//without the need of storing it
#[get("/")]
pub fn get_random_image(conn: DBPool) -> Result<ImageResponder, status::BadRequest<&'static str>> {
    RequestService::get_photo_url()
        .and_then(|url| {
            dbService::insert_entry(url.clone(), conn)?;
            RequestService::get_image_from_url(url) 
        })
        .map(|v| ImageResponder::new(v))
        .map_err(|e| status::BadRequest(Some(e)))
}