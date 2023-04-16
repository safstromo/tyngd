#![feature(proc_macro_hygiene, decl_macro)]
#![warn(clippy::all,clippy::pedantic)]

use std::collections::HashMap;
use std::env;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::{routes, Rocket, Build};

#[macro_use]
extern crate rocket;

use rocket::get;
use rocket::http::Status;
use rocket::post;

use rocket::response::status;
use rocket::response::status::Created;

// use rocket_contrib::json::Json;
use rocket_dyn_templates::Template;
use serde::de::Unexpected::Str;
use rocket::serde::json::{Json, Value, json};


use crate::exercise::{Exercise, NewExercise};

mod exercise;
mod schema;

#[get("/")]
async fn index() -> Template {
    let context: HashMap<i32, i32> = HashMap::new();
    Template::render("home", context)

}


#[get("/api", format = "json")]
async fn get_all() -> Option<Json<Vec<Exercise>>> {
    let exercises = Exercise::get_all(&mut establish_connection());
    Some(Json(exercises))
}

#[get("/api/<id>", format = "json")]
async fn get_by_id(id: i32) -> Option<Json<Exercise>> {
    let exercise = Exercise::get_exercise_by_id(&id, &mut establish_connection());
    Some(Json(exercise.ok().unwrap()))
}

#[post("/new", format = "json", data = "<new_exercise>")]
async fn new_exercise(new_exercise: Json<NewExercise>) -> Created<Json<Exercise>> {

    //TODO Check if exists and if exist update

    let connection = &mut establish_connection();
    let exercise_name = String::from(&new_exercise.name);
    Exercise::insert_exercise(new_exercise.into_inner(), connection);
    let added_exercise = Exercise::get_exercise_by_name(&exercise_name, connection);
    let url = format!("/api/{}", added_exercise.name);
    Created::new(url).body(Json(added_exercise))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, get_all, new_exercise,get_by_id])
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


// #[cfg(test)]
// mod test {
//     use rocket::http::Status;
//     use rocket::local::Client;
//     use crate::rocket;
//
//     //TODO More tests!
//
//     #[test]
//     fn index() {
//         let client = Client::new(rocket()).expect("Rocket");
//         let response = client.get("/").dispatch();
//         assert_eq!(response.status(), Status::Ok)
//     }
// }
