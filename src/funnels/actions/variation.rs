use crate::database::establish_connection;
use diesel::prelude::*;
use crate::schema::variations;
use crate::funnels::models::variation::{
    NewVariation,
    Variation,
};
use crate::funnels::requests::new_variation::NewVariationRequest;

pub fn create_variation(variation_request: NewVariationRequest) -> Variation {
    let conn = &mut establish_connection();
    let new_variation = NewVariation {
        label: &variation_request.label,
        funnel_id: variation_request.funnel_id,
        created_at: std::time::SystemTime::now()
    };
    diesel::insert_into(variations::table)
        .values(&new_variation)
        .get_result(conn)
        .expect("Error saving new funnel")
}

pub fn get_all_active_variations() -> Vec<Variation> {
    let conn = &mut establish_connection();
    variations::table
        .load::<Variation>(conn)
        .expect("Error loading post")
}

pub fn get_all_active_variations_from_funnel_id(id: i32) -> Vec<Variation> {
    let conn = &mut establish_connection();
    variations::table
        .filter(variations::funnel_id.eq(id))
        .load::<Variation>(conn)
        .expect("Error loading post")
}
