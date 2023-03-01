use rocket::serde::Serialize;
use crate::funnels::models::step::Step;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StepResponse {
    pub id: i32,
    pub title: String,
    pub variation_id: i32,
    pub order: i32,
}

pub trait FromStep {
    fn from_step(step: Step) -> Self;
}

impl FromStep for StepResponse {
    fn from_step(step: Step) -> Self {
        Self {
            id: step.id,
            title: step.title.to_owned(),
            variation_id: step.variation_id,
            order: step.order,
        }
    }
}
