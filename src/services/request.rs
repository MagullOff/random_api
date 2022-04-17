use reqwest::blocking::get;
use rand::seq::SliceRandom;
use crate::types::{self, config::Config};

pub fn get_photo_url() -> Result<String,&'static str> {
    get(gen_request_url())
        .and_then(|r| r.json::<types::request::Resp>())
        .map(|r| {
            r.data.children
                .into_iter()
                .filter_map(|p| p.data.preview.and_then(|prev| {
                    prev
                        .images
                        .first()
                        .and_then(|i| Some(i.source.url.clone()))
                }))
                .collect::<Vec<String>>()
        })
        .map_err(|_| "Error requesting images")
        .and_then(|v| {
            match v.is_empty() {
                true => Err("No photos on the subreddit"),
                false => { 
                    match v.choose(&mut rand::thread_rng()) {
                        Some(s) => Ok(s.clone()),
                        None => Err("Error choosing photo")
                    }
                }
            }
        })
}

pub fn get_image_from_url(photo_url: String) -> Result<Vec<u8>,&'static str> {
    get(photo_url)
        .and_then(|r| r.bytes())
        .and_then(|b| Ok(b.to_vec()))
        .map_err(|_| "Error fetching the image")

}

fn gen_request_url() -> String {
    format!("https://reddit.com/r/{}/hot.json?raw_json=1",Config::get_subreddit())
}