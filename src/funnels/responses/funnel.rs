use rocket::serde::Serialize;
use crate::funnels::responses::variation::VariationResponse;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FunnelResponse {
    pub id: i32,
    pub label: String,
    pub slug: String,
    pub variations: Option<Vec<VariationResponse>>,
}
