use diesel::prelude::*;
use std::time::SystemTime;
use crate::schema::contents;

#[derive(Queryable, Identifiable, Selectable)]
pub struct Content {
    pub id: i32,
    pub content: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub grapesjs: Option<String>,
}
