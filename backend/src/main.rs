#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::response::content::RawHtml;
use crate::models::tipper::Tipper;
use crate::routes::tippers::*;

mod routes;
mod models;

pub type TipperStore = tokio::sync::Mutex<Vec<Tipper>>;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("../frontend/dist")))
        .mount("/", routes![list, add, update, delete])
        .manage(TipperStore::new( vec![]))
}
