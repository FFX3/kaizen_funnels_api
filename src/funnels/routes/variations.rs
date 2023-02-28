use rocket::serde::json::Json;
use crate::funnels::requests::new_variation::NewVariationRequest;
use crate::funnels::responses::variation::VariationResponse;
use crate::funnels::actions::variation::{
    create_variation,
    get_all_active_variations
};
use crate::funnels::responses::variation::*;


#[post("/", data = "<variation_request>")]
pub fn create(variation_request: Json<NewVariationRequest>) -> Json<VariationResponse> {
    let variation = create_variation(variation_request.into_inner());
    Json(VariationResponse {
        id: variation.id,
        funnel_id: variation.funnel_id,
        label: variation.label.to_owned()
    })
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
