#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
pub mod api;
pub mod types;
pub mod services;
pub mod schema;
pub mod repository;