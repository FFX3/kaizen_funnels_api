use diesel::prelude::*;
use std::time::SystemTime;
use crate::schema::contents;

#[derive(Queryable, Identifiable, Selectable)]
pub struct Content {
    pub id: i32,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = contents)]
pub struct NewContent <'a> {
    pub content: &'a str,
    pub created_at: SystemTime
}
