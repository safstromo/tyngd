#![feature(proc_macro_hygiene, decl_macro)]

mod exercise;
mod schema;

use std::fmt::format;
use rocket::http::hyper::StatusCode;
use rocket::http::hyper::StatusCode::Created;
use rocket::{Response, Rocket};
use rocket::response::{ResponseBuilder, status};
use serde::{Serialize, Deserialize};
use crate::exercise::Exercise;
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

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index,new_exercise])
}

fn main() {
    rocket().launch();
    let stuff: Vec<Exercise> = Vec::new();
}

mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use crate::main;

    #[test]
    fn index() {
        let client = Client::new(rocket()).expect("Rocket");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok)
    }
}