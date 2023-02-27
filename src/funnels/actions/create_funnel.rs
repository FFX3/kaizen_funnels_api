use diesel::PgConnection;
use diesel::prelude::*;
use crate::schema::funnels;
use crate::funnels::models::funnel::{
    NewFunnel,
    Funnel,
};
use crate::funnels::request::new_funnel::NewFunnelRequest;

pub fn create_funnel(conn: &mut PgConnection, funnel_request: NewFunnelRequest) -> Funnel {
    let new_post = NewFunnel {
        label: &funnel_request.label,
        created_at: std::time::SystemTime::now()
    };
    diesel::insert_into(funnels::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new funnel")
}