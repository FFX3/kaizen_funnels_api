use rocket::serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewFunnelRequest {
    pub label: String
}