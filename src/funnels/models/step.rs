use diesel::prelude::*;
use std::time::SystemTime;
use crate::schema::steps;
use crate::funnels::models::variation::Variation;

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(Variation))]
#[diesel(table_name = steps)]
pub struct Step {
    pub id: i32,
    pub title: String,
    pub variation_id: i32,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>
}

#[derive(Insertable)]
#[diesel(table_name = steps)]
pub struct NewStep<'a> {
    pub title: &'a str,
    pub created_at: SystemTime,
    pub variation_id: i32,
}
