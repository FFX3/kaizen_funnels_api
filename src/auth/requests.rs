use rocket::serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUserRequest {
    pub email: String,
    pub password: String,
    pub organization_id: i32
}
