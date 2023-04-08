// @generated automatically by Diesel CLI.

diesel::table! {
    exercise (ExerciseId) {
        ExerciseId -> Integer,
        Name -> Nullable<Varchar>,
        Description -> Nullable<Text>,
        Weight -> Nullable<Integer>,
        Reps -> Nullable<Integer>,
        ExSet -> Nullable<Integer>,
        Video -> Nullable<Text>,
    }
}
