use crate::database::establish_connection;
use crate::funnels::requests::new_variation::NewVariationRequest;
use diesel::prelude::*;
use crate::schema::tenant::funnels;
use crate::funnels::models::funnel::{
    NewFunnel,
    Funnel,
};
use crate::funnels::requests::new_funnel::{
    NewFunnelRequest,
    UpdateFunnelRequest
};

use super::variation::{ get_variation_by_id, clone_variation};

pub fn get_all_active_funnels() -> Vec<Funnel> {
    let conn = &mut establish_connection();
    funnels::table
        .filter(funnels::deleted_at.is_null())
        .load::<Funnel>(conn)
        .expect("Error loading post")
}

pub fn get_funnel_by_id(id: i32) -> Option<Funnel> {
    let conn = &mut establish_connection();
    funnels::table
        .filter(funnels::id.eq(id))
        .first(conn)
        .optional()
        .expect("Error loading funnel")
}

pub fn create_funnel(funnel_request: &NewFunnelRequest) -> Funnel {
    let conn = &mut establish_connection();
    let new_funnel = NewFunnel {
        label: &funnel_request.label,
        slug: &funnel_request.slug,
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

pub fn create_funnel_with_cloned_variation(variation_id: i32, funnel_request: &NewFunnelRequest) -> Option<Funnel> {
    let reference_variation = get_variation_by_id(variation_id);
    if reference_variation.is_none() {
        return None;
    }
    let reference_variation = reference_variation.unwrap();

    let new_funnel = create_funnel(funnel_request);

    let variation_request = NewVariationRequest::clone_from_variation(&reference_variation, new_funnel.id);
    clone_variation(reference_variation.id, &variation_request);

    Some(new_funnel)
}
