#![feature(proc_macro_hygiene, decl_macro)]

mod exercise;

use serde::{Serialize, Deserialize};
use crate::exercise::Exercise;
use serde_json::*;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    let clean = Exercise::new("Clean".to_string());
serde_json::to_string(&clean).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}