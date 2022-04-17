use rocket::{http::ContentType, response::Responder};

#[derive(Responder)]
#[response(status = 200, content_type = "image/jpg")]
pub struct ImageResponder {
    inner: Vec<u8>,
    header: ContentType,
}

impl ImageResponder {
    pub fn new(vec: Vec<u8>) -> ImageResponder {
        ImageResponder {
            inner: vec,
            header: ContentType::JPEG,
        }
    }
}