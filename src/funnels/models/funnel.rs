use diesel::prelude::*;
use std::time::SystemTime;
use crate::schema::tenant::funnels;

#[derive(Queryable, Identifiable, Selectable)]
pub struct Funnel {
    pub id: i32,
    pub label: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub slug: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = funnels)]
pub struct NewFunnel<'a> {
    pub label: &'a str,
    pub slug: &'a str,
    pub created_at: SystemTime
}
