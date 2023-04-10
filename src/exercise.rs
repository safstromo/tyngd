use super::schema::exercise::dsl::exercise as all_exercises;
use crate::schema::exercise;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use crate::schema::exercise::name;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = exercise)]
pub struct NewExercise {
    pub name: String,
    pub description: String,
    pub weight: i32,
    pub reps: i32,
    pub ex_set: i32,
    pub video: String,
}
#[derive(Serialize, Deserialize, Queryable, Debug, Eq, PartialEq)]
pub struct Exercise {
    exercise_id: i32,
    name: String,
    description: String,
    weight: i32,
    reps: i32,
    ex_set: i32,
    video: String,
}

impl Exercise {

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_all(connection: &mut MysqlConnection) -> Vec<Exercise> {
        all_exercises.load::<Exercise>(connection).expect("error")
    }
    pub fn get_exercise_by_name(exercise_name: &String, connection: &mut MysqlConnection) -> Vec<Exercise> {
        all_exercises
            .filter(name.eq(exercise_name))
            .load(connection)
            .expect("Error getting by name")
    }

    pub fn insert_exercise(exercise: NewExercise, connection: &mut MysqlConnection) -> bool {
        diesel::insert_into(exercise::table)
            .values(&exercise)
            .execute(connection)
            .is_ok()
    }
}
