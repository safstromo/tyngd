#![feature(proc_macro_hygiene, decl_macro)]

mod exercise;
mod schema;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::exercise::{Exercise, NewExercise};
use crate::schema::exercise::table;
use dotenv::dotenv;
use std::env;
use diesel::insert_into;
use diesel::mysql::MysqlConnection;
use crate::schema::exercise::dsl::{exercise as other_exercise};
use diesel::associations::HasTable;
use diesel::sql_types::Json;
use rocket::{Rocket, routes};
use serde_json::{json, Value};
use rocket_db_pools::Database;


// #[get("/")]
// fn index(connection: &mut MysqlConnection) -> Json {
//     let clean = Exercise::new("Clean".to_string());
//     let exercises = Exercise::get_all(connection);
//     Json(json!({
//         "status": 200,
//         "result": exercises,
//     }))
// }

// #[post("/new", format = "application/json", data = "<new_exercise>")]
// fn new_exercise(connection: MysqlConnection, new_exercise: Json) -> Json {
//     Json(json!({
//         "status": Exercise::insert_exercise(new_exercise.into_inner(), &connection),
//         "result": Exercise::get_all(&connection).first(),
//     }))
// }

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![/*index,new_exercise*/])
}

fn main() {
    let test = NewExercise {
        name: &"test".to_string(),
        description: &"tst".to_string(),
        weight: 0,
        reps: 0,
        ex_set: 0,
        video: &"video".to_string(),
    };
    let result = Exercise::insert_exercise(test, &mut establish_connection());
    println!("{}", result);

    let list = Exercise::get_all(&mut establish_connection());

    for ex in list {
        println!("{}", ex.name())
    }
    rocket()
        .launch();
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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