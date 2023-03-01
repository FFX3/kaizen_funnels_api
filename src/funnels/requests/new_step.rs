use rocket::serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewStepRequest {
    pub title: String,
    pub variation_id: i32
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateStepRequest {
    pub title: String,
}
