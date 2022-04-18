use rocket::config::{Config, Environment, Value};
use rocket::http::Method;
use rocket::{Rocket,routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::collections::HashMap;
use crate::types::DBPool;
use crate::types::config::Config as ConfigStruct;

fn get_cors() -> CorsOptions {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true)
}

fn build_config(config: ConfigStruct) -> Config {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", config.db_url);
    databases.insert("api-db", Value::from(database_config));
    let port_number: u16 = config.rocket_port;

    Config::build(Environment::Staging)
        .port(port_number)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

pub fn init() -> Rocket {
    
    let file_config = ConfigStruct::get_config();

    let config = build_config(file_config);

    let cors = get_cors();

    rocket::custom(config)
        .attach(cors.to_cors().unwrap())
        .attach(DBPool::fairing())
        .mount("/history", routes![super::history::get_history])
        .mount("/random", routes![super::random::get_random_image])
}
