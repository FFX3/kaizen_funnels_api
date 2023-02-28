use rocket::serde::Serialize;
use crate::funnels::models::variation::Variation;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VariationResponse {
    pub id: i32,
    pub label: String,
    pub funnel_id: i32,
}

pub trait FromVariation {
    fn from_variation(variation: Variation) -> Self;
}

impl FromVariation for VariationResponse {
    fn from_variation(variation: Variation) -> Self {
        Self {
            id: variation.id,
            label: variation.label.to_owned(),
            funnel_id: variation.funnel_id
        }
    }
}
