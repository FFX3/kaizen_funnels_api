use rocket::serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewFunnelRequest {
    pub label: String
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateFunnelRequest {
    pub label: String
}
