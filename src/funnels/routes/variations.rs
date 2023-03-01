use rocket::serde::json::Json;
use crate::funnels::requests::new_variation::NewVariationRequest;
use crate::funnels::responses::variation::VariationResponse;
use crate::funnels::actions::variation::{
    create_variation,
    get_all_active_variations,
    soft_delete_variation, 
    mark_variation_as_a, 
    mark_variation_as_b, 
    mark_variation_as_winner
};
use crate::funnels::responses::variation::*;


#[post("/", data = "<variation_request>")]
pub fn create(variation_request: Json<NewVariationRequest>) -> Json<VariationResponse> {
    let variation = create_variation(variation_request.into_inner());
    Json(VariationResponse::from_variation(variation))
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
