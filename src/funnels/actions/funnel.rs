use crate::database::establish_connection;
use diesel::prelude::*;
use crate::schema::funnels;
use crate::funnels::models::funnel::{
    NewFunnel,
    Funnel,
};
use crate::funnels::requests::new_funnel::{
    NewFunnelRequest,
    UpdateFunnelRequest
};

pub fn get_all_active_funnels() -> Vec<Funnel> {
    let conn = &mut establish_connection();
    funnels::table
        .load::<Funnel>(conn)
        .expect("Error loading post")
}


pub fn create_funnel(funnel_request: NewFunnelRequest) -> Funnel {
    let conn = &mut establish_connection();
    let new_funnel = NewFunnel {
        label: &funnel_request.label,
        created_at: std::time::SystemTime::now()
    };
    diesel::insert_into(funnels::table)
        .values(&new_funnel)
        .get_result(conn)
        .expect("Error saving new funnel")
}

pub fn update_funnel(id: i32, funnel_request: UpdateFunnelRequest) -> () {
    let conn = &mut establish_connection();
    diesel::update(funnels::table)
        .filter(funnels::id.eq(id))
        .filter(funnels::deleted_at.is_null())
        .set((
            funnels::label.eq(funnel_request.label),
            funnels::updated_at.eq(std::time::SystemTime::now())
        ))
        .execute(conn)
        .expect("Error updating funne");
}

pub fn soft_delete_funnel(id: i32) -> () {
    let conn = &mut establish_connection();
    diesel::update(funnels::table)
        .filter(funnels::id.eq(id))
        .filter(funnels::deleted_at.is_null())
        .set(funnels::deleted_at.eq(std::time::SystemTime::now()))
        .execute(conn)
        .expect("Error soft deleting funnel");
}
