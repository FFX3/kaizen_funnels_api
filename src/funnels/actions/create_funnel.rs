use crate::database::establish_connection;
use diesel::prelude::*;
use crate::schema::funnels;
use crate::funnels::models::funnel::{
    NewFunnel,
    Funnel,
};
use crate::funnels::requests::new_funnel::NewFunnelRequest;

pub fn create_funnel(funnel_request: NewFunnelRequest) -> Funnel {
    let conn = &mut establish_connection();
    let new_post = NewFunnel {
        label: &funnel_request.label,
        created_at: std::time::SystemTime::now()
    };
    diesel::insert_into(funnels::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new funnel")
}
