// @generated automatically by Diesel CLI.

diesel::table! {
    exercise (exercise_id) {
        exercise_id -> Integer,
        name -> Varchar,
        description -> Text,
        weight -> Integer,
        reps -> Integer,
        ex_set -> Integer,
        video -> Text,
    }
}
