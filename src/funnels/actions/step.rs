use crate::database::establish_connection;
use diesel::prelude::*;
use crate::schema::{ steps, contents };
use crate::funnels::models::step::{
    NewStep,
    Step,
};
use crate::funnels::models::content::Content;
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

pub fn get_step_content(id: i32) -> Option<String> {
    let conn = &mut establish_connection();
    contents::table
        .inner_join(steps::table)
        .filter(steps::deleted_at.is_null())
        .filter(steps::id.eq(id))
        .select(contents::content)
        .get_result::<Option<String>>(conn)
        .expect("Error fetching content")
}

pub fn get_step_grapesjs(id: i32) -> Option<String> {
    let conn = &mut establish_connection();
    contents::table
        .inner_join(steps::table)
        .filter(steps::deleted_at.is_null())
        .filter(steps::id.eq(id))
        .select(contents::grapesjs)
        .get_result::<Option<String>>(conn)
        .expect("Error fetching content")

}

pub fn update_step_content(id: i32, content_request: NewContentRequest) -> () {
    let conn = &mut establish_connection();

    diesel::update(contents::table)
        .filter(
            steps::table
                .filter(steps::deleted_at.is_null())
                .filter(steps::id.eq(id))
                .filter(steps::content_id.eq(contents::id))
                .count()
                .single_value()
                .eq(1)
        )
        .set(&content_request)
        .execute(conn)
        .expect("Error saving new content");
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
    };


    let content: Content = diesel::insert_into(contents::table)
        .values(contents::created_at.eq(std::time::SystemTime::now()))
        .get_result(conn)
        .expect("Error creating content entry");

    diesel::insert_into(steps::table)
        .values((
            &new_step, 
            steps::content_id.eq(content.id),
            steps::created_at.eq(std::time::SystemTime::now())
        ))
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
