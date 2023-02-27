use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FunnelResponse {
    pub id: i32,
    pub label: String
}
