use diesel::prelude::*;
use std::time::SystemTime;
use crate::schema::tenant::variations;
use crate::funnels::models::funnel::Funnel;

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(Funnel))]
#[diesel(table_name = variations)]
pub struct Variation {
    pub id: i32,
    pub label: String,
    pub funnel_id: i32,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub mark_a: Option<SystemTime>,
    pub mark_b: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = variations)]
pub struct NewVariation {
    pub label: String,
    pub funnel_id: i32,
    pub created_at: SystemTime
}
