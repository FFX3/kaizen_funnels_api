use rocket::serde::Serialize;
use crate::funnels::models::variation::Variation;
use std::time::SystemTime;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VariationResponse {
    pub id: i32,
    pub label: String,
    pub funnel_id: i32,
    pub mark_a: Option<SystemTime>,
    pub mark_b: Option<SystemTime>,
}

pub trait FromVariation {
    fn from_variation(variation: Variation) -> Self;
}

impl FromVariation for VariationResponse {
    fn from_variation(variation: Variation) -> Self {
        Self {
            id: variation.id,
            label: variation.label.to_owned(),
            funnel_id: variation.funnel_id,
            mark_a: variation.mark_a,
            mark_b: variation.mark_b,
        }
    }
}
