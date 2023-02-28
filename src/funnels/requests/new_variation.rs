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
