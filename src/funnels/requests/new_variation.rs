use rocket::serde::{
    Deserialize,
    Serialize,
};
use crate::funnels::models::variation::Variation;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewVariationRequest {
    pub label: String,
    pub funnel_id: i32
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateVariationRequest {
    pub label: String,
    pub step_order: Option<Vec<i32>>,
}


impl NewVariationRequest {
    pub fn clone_from_variation(variation: &Variation, funnel_id: i32) -> NewVariationRequest {
        NewVariationRequest { 
            label: variation.label.to_string(),
            funnel_id
        }
    }
}
