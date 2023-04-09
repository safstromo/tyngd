use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel;
use super::schema::exercise::dsl::exercise as all_exercises;
use crate::schema::exercise;


#[derive(Insertable)]
#[diesel(table_name = exercise)]
pub struct NewExercise <'a>{
  pub  name: &'a String,
   pub description: &'a String,
    pub weight: i32,
    pub reps: i32,
    pub ex_set: i32,
    pub video: &'a String,

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
    pub fn new(name: String) -> Exercise {
        Exercise {
            exercise_id: 0,
            name,
            description: "".to_string(),
            weight: 0,
            reps: 0,
            ex_set: 0,
            video: "".to_string(),
        }
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_all(connection: &mut MysqlConnection) -> Vec<Exercise> {
        all_exercises
            .load::<Exercise>(connection)
            .expect("error")

    }
    // pub fn get_exercise_by_name(exercise: Exercise, connection: &MysqlConnection) -> Vec<Exercise>{
    //     all_exercises
    //         .filter(exercise::name)
    //         .load::<Exercise>(connection)
    //         .expect("Unable to find")
    // }
    //
    pub fn insert_exercise(exercise: NewExercise, connection: &mut MysqlConnection) -> bool {
        diesel::insert_into(exercise::table)
            .values(&exercise)
            .execute(connection)
            .is_ok()
    }
}
