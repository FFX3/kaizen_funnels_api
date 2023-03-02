use diesel::AsChangeset;
use rocket::serde::{
    Deserialize,
    Serialize
};
use crate::schema::contents;

#[derive(Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = contents)]
#[serde(crate = "rocket::serde")]
pub struct NewContentRequest {
    pub content: Option<String>,
    pub grapesjs: Option<String>,
}
