#![feature(proc_macro_hygiene, decl_macro)]

mod exercise;

use std::fmt::format;
use rocket::http::hyper::StatusCode;
use rocket::http::hyper::StatusCode::Created;
use rocket::Response;
use rocket::response::{ResponseBuilder, status};
use serde::{Serialize, Deserialize};
use crate::exercise::Exercise;
use serde_json::*;
use rocket::http::Status;


#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    let clean = Exercise::new("Clean".to_string());
    serde_json::to_string(&clean).unwrap()
}

#[post("/new", format = "application/json", data = "<exercise>")]
fn new_exercise(exercise: String) -> status::Created<String> {
    let exercise: Exercise = serde_json::from_str(&exercise).unwrap();
    status::Created("/new".to_string(), Some(serde_json::to_string(&exercise).unwrap()))
}

fn main() {
    rocket::ignite().mount("/", routes![index,new_exercise]).launch();
    let stuff: Vec<Exercise> = Vec::new();
}