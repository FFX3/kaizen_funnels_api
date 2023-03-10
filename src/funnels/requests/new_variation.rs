use rocket::serde::{
    Deserialize,
    Serialize,
};

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
