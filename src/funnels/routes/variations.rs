use rocket::serde::json::Json;
use crate::funnels::requests::new_variation::{
    NewVariationRequest,
    UpdateVariationRequest
};
use crate::funnels::responses::variation::VariationResponse;
use crate::funnels::responses::step::*;
use crate::funnels::actions::variation::{
    get_variation_by_id,
    create_variation,
    update_variation,
    get_all_active_variations,
    soft_delete_variation, 
    mark_variation_as_a, 
    mark_variation_as_b, 
    mark_variation_as_winner,
    reorder_variation_steps,
};
use crate::funnels::actions::step::get_all_steps_from_variation_id;
use crate::funnels::responses::variation::*;

#[get("/<id>/steps")]
pub fn list_steps(id: i32) -> Json<Vec<StepResponse>> {
    let step_list = get_all_steps_from_variation_id(id);
    let response = step_list
        .into_iter()
        .map(|step| (StepResponse::from_step(step)))
        .collect();
    Json(response)
}

#[post("/", data = "<variation_request>")]
pub fn create(variation_request: Json<NewVariationRequest>) -> Json<VariationResponse> {
    let variation = create_variation(variation_request.into_inner());
    Json(VariationResponse::from_variation(variation))
}

#[put("/<id>", data = "<funnel_request>")]
pub fn update(id: i32, funnel_request: Json<UpdateVariationRequest>) -> () {
    update_variation(id, funnel_request.into_inner());
}

#[get("/")]
pub fn list() -> Json<Vec<VariationResponse>> {
    let variation_list = get_all_active_variations();
    let response = variation_list
        .into_iter()
        .map(|variation| (VariationResponse::from_variation(variation)))
        .collect();
    Json(response)
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Json<VariationResponse> {
    let variation = get_variation_by_id(id);
    let mut variation_response = VariationResponse::from_variation(variation.expect("figure out how to handle not found"));
    let step_list = get_all_steps_from_variation_id(id);
    let step_response_list: Vec<StepResponse> = step_list
        .into_iter()
        .map(|step| (StepResponse::from_step(step)))
        .collect();
    variation_response.steps = step_response_list;
    Json(variation_response)
}

#[delete("/<id>")]
pub fn delete(id: i32) -> () {
    soft_delete_variation(id);
}

#[patch("/a/<id>")]
pub fn mark_a(id: i32) -> () {
    mark_variation_as_a(id);
}

#[patch("/b/<id>")]
pub fn mark_b(id: i32) -> () {
    mark_variation_as_b(id);
}

#[patch("/winner/<id>")]
pub fn mark_winner(id: i32) -> () {
    mark_variation_as_winner(id);
}

#[patch("/reorder-steps/<id>", data="<order>")]
pub fn reorder_steps(id: i32, order: Json<Vec<i32>>) -> () {
    reorder_variation_steps(id, order.into_inner());
}
