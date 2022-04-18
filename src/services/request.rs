use reqwest::blocking::get;
use rand::seq::SliceRandom;
use crate::types::{self, config::Config, request::Resp};

pub fn get_photo_url() -> Result<String,&'static str> {
    get(gen_request_url(Config::get_subreddit()))
        .and_then(|r| r.json::<types::request::Resp>())
        .map(get_url_vec)
        .map_err(|_| "Error requesting images")
        .and_then(get_random_image)
}

pub fn get_image_from_url(photo_url: String) -> Result<Vec<u8>,&'static str> {
    get(photo_url)
        .and_then(|r| r.bytes())
        .and_then(|b| Ok(b.to_vec()))
        .map_err(|_| "Error fetching the image")
}

fn get_random_image(url_vector: Vec<String>) -> Result<String,&'static str> {
    match url_vector.is_empty() {
        true => Err("No photos on the subreddit"),
        false => { 
            url_vector.choose(&mut rand::thread_rng())
                .map(|url| url.to_string())
                .ok_or("Error choosing the photo")
        }
    }
}

fn get_url_vec(resp: Resp) -> Vec<String> {
    resp.data.children
        .into_iter()
        .filter_map(|p| p.data.preview.and_then(|prev| {
            prev
                .images
                .first()
                .and_then(|img| Some(img.source.url.clone()))
        }))
        .collect::<Vec<String>>()
}

fn gen_request_url(tag: String) -> String {
    format!("https://reddit.com/r/{}/hot.json?raw_json=1",tag)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_url_generator(){
        assert_eq!("https://reddit.com/r/rust/hot.json?raw_json=1".to_string(),gen_request_url("rust".to_string()));
    }
}