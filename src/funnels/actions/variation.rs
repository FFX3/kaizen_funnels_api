use crate::database::establish_connection;
use diesel::prelude::*;
use diesel::sql_query;
use crate::schema::variations;
use crate::funnels::models::variation::{
    NewVariation,
    Variation,
};
use crate::funnels::requests::new_variation::{
    NewVariationRequest,
    UpdateVariationRequest
};

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
        .expect("Error saving new variation")
}

pub fn update_variation(id: i32, variation_request: &UpdateVariationRequest) -> () {
    let conn = &mut establish_connection();
    diesel::update(variations::table)
        .filter(variations::id.eq(id))
        .filter(variations::deleted_at.is_null())
        .set((
            variations::label.eq(&variation_request.label.to_string()),
            variations::updated_at.eq(std::time::SystemTime::now())
        ))
        .execute(conn)
        .expect("Error soft deleting variation");
}

pub fn get_all_active_variations() -> Vec<Variation> {
    let conn = &mut establish_connection();
    variations::table
        .filter(variations::deleted_at.is_null())
        .load::<Variation>(conn)
        .expect("Error loading variation")
}

pub fn get_variation_by_id(id: i32) -> Option<Variation> {
    let conn = &mut establish_connection();
    variations::table
        .filter(variations::id.eq(id))
        .first(conn)
        .optional()
        .expect("Error loading funnel")
}

pub fn get_all_active_variations_from_funnel_id(id: i32) -> Vec<Variation> {
    let conn = &mut establish_connection();
    variations::table
        .filter(variations::deleted_at.is_null())
        .filter(variations::funnel_id.eq(id))
        .load::<Variation>(conn)
        .expect("Error loading variation")
}

pub fn get_ab_variations_by_funnel_id(id: i32) -> (Variation, Variation) {
    let conn = &mut establish_connection();
    let a = variations::table
        .filter(variations::funnel_id.eq(id))
        .filter(variations::mark_a.is_not_null())
        .order(variations::mark_a.desc())
        .limit(1)
        .get_result(conn)
        .expect("Error loading variation");
    let b = variations::table
        .filter(variations::funnel_id.eq(id))
        .filter(variations::mark_b.is_not_null())
        .order(variations::mark_b.desc())
        .limit(1)
        .get_result(conn)
        .expect("Error loading variation");
    (a, b)
}

pub fn soft_delete_variation(id: i32) -> () {
    let conn = &mut establish_connection();
    diesel::update(variations::table)
        .filter(variations::id.eq(id))
        .filter(variations::deleted_at.is_null())
        .set(variations::deleted_at.eq(std::time::SystemTime::now()))
        .execute(conn)
        .expect("Error soft deleting variation");
}

pub fn mark_variation_as_a(id: i32) -> () {
    let conn = &mut establish_connection();
    diesel::update(variations::table)
        .filter(variations::id.eq(id))
        .filter(variations::deleted_at.is_null())
        .set(variations::mark_a.eq(std::time::SystemTime::now()))
        .execute(conn)
        .expect("Error marking variation as a");
}

pub fn mark_variation_as_b(id: i32) -> () {
    let conn = &mut establish_connection();
    diesel::update(variations::table)
        .filter(variations::id.eq(id))
        .filter(variations::deleted_at.is_null())
        .set(variations::mark_b.eq(std::time::SystemTime::now()))
        .execute(conn)
        .expect("Error marking variation as b");
}

pub fn mark_variation_as_winner(id: i32) -> () {
    let conn = &mut establish_connection();
    diesel::update(variations::table)
        .filter(variations::id.eq(id))
        .filter(variations::deleted_at.is_null())
        .set((
            variations::mark_a.eq(std::time::SystemTime::now()),
            variations::mark_b.eq(std::time::SystemTime::now())
        ))
        .execute(conn)
        .expect("Error marking variation as winner");
}

pub fn reorder_variation_steps(id: i32, order: Vec<i32>) -> () {
    let mut query_string = String::from("UPDATE steps SET \"order\" = (CASE ");
    for (index, step_id) in order.iter().enumerate() {
        query_string.push_str(" WHEN id=");
        query_string.push_str(&step_id.to_string());
        query_string.push_str(" THEN ");
        query_string.push_str(&index.to_string());
    }
    query_string.push_str(" END)");
    query_string.push_str("WHERE variation_id=");
    query_string.push_str(&id.to_string());
    query_string.push_str(" AND id IN (");
    let mut ins = order.iter().map(|id| id.to_string() + ",").collect::<String>();
    ins.pop();
    query_string.push_str(&ins);
    query_string.push_str(");");
    let conn = &mut establish_connection();
    println!("{}", query_string);
    _ = sql_query(query_string).execute(conn);
}
