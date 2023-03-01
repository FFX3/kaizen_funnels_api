use crate::database::establish_connection;
use diesel::prelude::*;
use crate::schema::steps;
use crate::funnels::models::step::{
    NewStep,
    Step,
};
use crate::funnels::requests::new_step::{
    NewStepRequest,
    UpdateStepRequest
};

pub fn get_all_active_steps() -> Vec<Step> {
    let conn = &mut establish_connection();
    steps::table
        .filter(steps::deleted_at.is_null())
        .load::<Step>(conn)
        .expect("Error loading post")
}


pub fn create_step(step_request: NewStepRequest) -> Step {
    let conn = &mut establish_connection();
    let new_step = NewStep {
        title: &step_request.title,
        variation_id: step_request.variation_id,
        created_at: std::time::SystemTime::now()
    };
    diesel::insert_into(steps::table)
        .values(&new_step)
        .get_result(conn)
        .expect("Error saving new step")
}

pub fn update_step(id: i32, step_request: UpdateStepRequest) -> () {
    let conn = &mut establish_connection();
    diesel::update(steps::table)
        .filter(steps::id.eq(id))
        .filter(steps::deleted_at.is_null())
        .set((
            steps::title.eq(step_request.title),
            steps::updated_at.eq(std::time::SystemTime::now())
        ))
        .execute(conn)
        .expect("Error updating funne");
}

pub fn soft_delete_step(id: i32) -> () {
    let conn = &mut establish_connection();
    diesel::update(steps::table)
        .filter(steps::id.eq(id))
        .filter(steps::deleted_at.is_null())
        .set(steps::deleted_at.eq(std::time::SystemTime::now()))
        .execute(conn)
        .expect("Error soft deleting step");
}
