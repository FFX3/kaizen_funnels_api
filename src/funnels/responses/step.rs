use rocket::serde::Serialize;
use crate::funnels::models::step::Step;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StepResponse {
    pub id: i32,
    pub title: String
}

pub trait FromStep {
    fn from_step(step: Step) -> Self;
}

impl FromStep for StepResponse {
    fn from_step(step: Step) -> Self {
        Self {
            id: step.id,
            title: step.title.to_owned()
        }
    }
}
