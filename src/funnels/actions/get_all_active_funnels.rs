use crate::database::establish_connection;
use diesel::prelude::*;
use crate::funnels::models::funnel::Funnel;
use crate::schema::funnels;

pub fn get_all_active_funnels() -> Vec<Funnel> {
    let conn = &mut establish_connection();
    funnels::table
        .load::<Funnel>(conn)
        .expect("Error loading post")
}
