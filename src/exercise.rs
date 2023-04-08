use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Exercise {
    id: i64,
    name: String,
    description: String,
    weight: i32,
    reps: i32,
    set: i32,
    video: String,
}

impl Exercise {
    pub fn new(name: String) -> Exercise {
        Exercise {
            id: 0,
            name,
            description: "".to_string(),
            weight: 0,
            reps: 0,
            set: 0,
            video: "".to_string(),
        }
    }
}