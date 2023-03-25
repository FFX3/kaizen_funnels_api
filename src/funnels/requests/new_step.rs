use rocket::serde::{
    Deserialize,
    Serialize,
};

use crate::funnels::models::step::Step;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewStepRequest {
    pub title: String,
    pub variation_id: i32,
    pub order: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateStepRequest {
    pub title: String,
    pub order: i32,
}

impl NewStepRequest {
    pub fn clone_from_step(step: &Step, variation_id: i32) -> NewStepRequest {
        NewStepRequest {
            title: step.title.to_string(),
            order: step.order,
            variation_id
        }
    }
}
