#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::HashMap;
use std::env;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::{routes, Rocket};

use rocket::get;
use rocket::post;

use rocket::response::status;

use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

use crate::exercise::{Exercise, NewExercise};

mod exercise;
mod schema;

#[get("/")]
fn index() -> Template {
    let context: HashMap<i32, i32> = HashMap::new();
    Template::render("home", context)
}

#[get("/api")]
fn get_all() -> Json<Vec<Exercise>> {
    let exercises = Exercise::get_all(&mut establish_connection());
    Json(exercises)
}

#[post("/new", format = "json", data = "<new_exercise>")]
fn new_exercise(new_exercise: Json<NewExercise>) -> status::Created<String> {
    let connection = &mut establish_connection();
    let exercise_name = String::from(&new_exercise.name);
    Exercise::insert_exercise(new_exercise.into_inner(), connection);
    let added_exercise = Exercise::get_exercise_by_name(&exercise_name, connection);
    //TODO Check if exists and if exist update
    status::Created(String::from("Created"), Some(serde_json::to_string(added_exercise.first().unwrap()).unwrap()))
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, get_all, new_exercise])
}

fn main() {
    let test = NewExercise {
        name: "test".to_string(),
        description: "tst".to_string(),
        weight: 0,
        reps: 0,
        ex_set: 0,
        video: "video".to_string(),
    };
    let result = Exercise::insert_exercise(test, &mut establish_connection());
    println!("{}", result);

    let list = Exercise::get_all(&mut establish_connection());

    for ex in list {
        println!("{:?}", ex)
    }
    rocket().launch();
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;
    use crate::rocket;

    #[test]
    fn index() {
        let client = Client::new(rocket()).expect("Rocket");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok)
    }
}
