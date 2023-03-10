use rocket::serde::json::Json;
use crate::funnels::requests::new_step::{
    NewStepRequest,
    UpdateStepRequest
};
use crate::funnels::requests::new_content::NewContentRequest;
use crate::funnels::responses::step::*;
use crate::funnels::actions::step::{
    get_all_active_steps,
    create_step, 
    update_step, 
    soft_delete_step,
    update_step_content,
    get_step_content,
    get_step_grapesjs,
    get_one_step,
};

#[get("/<id>/content")]
pub fn get_content(id: i32) -> Option<String> {
    get_step_content(id)
}

#[get("/<id>/grapesjs")]
pub fn get_grapesjs(id: i32) -> String {
    let grapesjs_cache = get_step_grapesjs(id);
    if let Some(cache) = grapesjs_cache {
        return cache;
    } else {
        return String::from("");
    }
}


#[get("/<id>")]
pub fn get_one(id: i32) -> Json<StepResponse> {
    Json(StepResponse::from_step(get_one_step(id)))
}

#[patch("/<id>/content", data = "<content_request>")]
pub fn update_content(id: i32, content_request: Json<NewContentRequest>) -> () {
    update_step_content(id, content_request.into_inner());
}

#[post("/", data = "<step_request>")]
pub fn create(step_request: Json<NewStepRequest>) -> Json<StepResponse> {
    let step = create_step(step_request.into_inner());
    Json(StepResponse::from_step(step))
}

#[put("/<id>", data = "<step_request>")]
pub fn update(id: i32, step_request: Json<UpdateStepRequest>) -> () {
    update_step(id, step_request.into_inner());
}

#[get("/")]
pub fn list() -> Json<Vec<StepResponse>> {
    let step_list = get_all_active_steps();
    let response = step_list
        .into_iter()
        .map(|step| (StepResponse::from_step(step)))
        .collect();
    Json(response)
}

#[delete("/<id>")]
pub fn delete(id: i32) -> () {
    soft_delete_step(id);
}
