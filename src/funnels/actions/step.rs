use crate::database::establish_connection;
use diesel::prelude::*;
use crate::schema::{ steps, contents };
use crate::funnels::models::step::{
    NewStep,
    Step,
};
use crate::funnels::models::content::{ NewContent, Content };
use crate::funnels::requests::new_step::{
    NewStepRequest,
    UpdateStepRequest
};

use crate::funnels::requests::new_content::NewContentRequest;

pub fn get_all_active_steps_from_variation_id(id: i32) -> Vec<Step> {
    let conn = &mut establish_connection();
    steps::table
        .filter(steps::deleted_at.is_null())
        .filter(steps::variation_id.eq(id))
        .order(steps::order)
        .load::<Step>(conn)
        .expect("Error loading variation")
}

pub fn update_step_content(id: i32, content_request: NewContentRequest) -> () {
    let new_content = NewContent {
        content: &content_request.content,
        created_at: std::time::SystemTime::now()
    };
    let conn = &mut establish_connection();
    let content: Content = diesel::insert_into(contents::table)
        .values(&new_content)
        .get_result(conn)
        .expect("Error saving new content");
    diesel::update(steps::table)
        .filter(steps::id.eq(id))
        .filter(steps::deleted_at.is_null())
        .set((
            steps::content_id.eq(content.id),
            steps::updated_at.eq(std::time::SystemTime::now())
        ))
        .execute(conn)
        .expect("Error updating step");
}

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
        order: step_request.order,
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
            steps::order.eq(step_request.order),
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
