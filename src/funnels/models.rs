use diesel::prelude::*;
use std::time::SystemTime;
use crate::schema::funnels;

#[derive(Queryable)]
pub struct Funnel {
    pub id: i32,
    pub label: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>
}

#[derive(Insertable)]
#[diesel(table_name = funnels)]
pub struct NewFunnel<'a> {
    pub label: &'a str,
    pub created_at: SystemTime
}