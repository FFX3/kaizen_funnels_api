use diesel::AsChangeset;
use rocket::serde::{
    Deserialize,
    Serialize
};
use crate::schema::tenant::contents;
use crate::funnels::models::content::Content;

#[derive(Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = contents)]
#[serde(crate = "rocket::serde")]
pub struct NewContentRequest {
    pub content: Option<String>,
    pub grapesjs: Option<String>,
}

impl NewContentRequest {
    pub fn clone_from_content(content: &Content) -> NewContentRequest {
        NewContentRequest { 
            content: content.content.to_owned(),
            grapesjs: content.grapesjs.to_owned()
        }
    }
}
