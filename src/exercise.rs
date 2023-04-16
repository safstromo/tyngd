use std::process::id;
use super::schema::exercise::dsl::exercise as all_exercises;
use crate::schema::exercise;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value};

// use serde::{Deserialize, Serialize};
use crate::schema::exercise::{exercise_id, name};

#[derive(Insertable, Deserialize, Queryable)]
#[diesel(table_name = exercise)]
pub struct NewExercise {
    pub name: String,
    pub description: String,
    pub video: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone, Debug, Eq, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Exercise {
    pub exercise_id: i32,
    pub name: String,
    description: String,
    video: String,
}

impl Exercise {

    pub fn get_all(connection: &mut MysqlConnection) -> Vec<Exercise> {
        all_exercises.load::<Exercise>(connection).expect("error")
    }
    pub fn get_exercise_by_name(exercise_name: &String, connection: &mut MysqlConnection) -> Exercise {
        let exercises_found: Vec<Exercise> = all_exercises
            .filter(name.eq(exercise_name))
            .load(connection)
            .expect("Error getting by name");

        exercises_found.first().unwrap().clone()
    }

    pub fn get_exercise_by_id(id: &i32, connection: &mut MysqlConnection) -> Result<Exercise, Error> {
        all_exercises
            .find(id)
            .first(connection)
    }

    pub fn insert_exercise(exercise: NewExercise, connection: &mut MysqlConnection) -> Exercise {
        diesel::insert_into(exercise::table)
            .values(&exercise)
            .execute(connection)
            .expect("Error saving");

        Exercise::get_exercise_by_name(&exercise.name, connection)
    }
}
