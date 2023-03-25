use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::Deserialize;
use crate::funnels::models::funnel::Funnel;
use crate::funnels::requests::new_funnel::{
    NewFunnelRequest,
    UpdateFunnelRequest
};
use crate::funnels::responses::funnel::FunnelResponse;
use crate::funnels::actions::funnel::{
    get_all_active_funnels,
    create_funnel, 
    update_funnel, 
    soft_delete_funnel, get_funnel_by_id, create_funnel_with_cloned_variation,
};
use crate::funnels::actions::variation::{get_all_active_variations_from_funnel_id, get_ab_variations_by_funnel_id};
use crate::funnels::responses::variation::*;

trait FromFunnel {
    fn from_funnel(funnel: Funnel) -> Self;
}

impl FromFunnel for FunnelResponse {
    fn from_funnel(funnel: Funnel) -> Self {
        Self {
            id: funnel.id,
            label: funnel.label.to_owned(),
            slug: if let Some(slug) = funnel.slug { slug } else { "".to_owned() },
            variations: None
        }
    }
}

#[post("/", data = "<funnel_request>")]
pub fn create(funnel_request: Json<NewFunnelRequest>) -> Json<FunnelResponse> {
    let funnel = create_funnel(&funnel_request.into_inner());
    Json(FunnelResponse::from_funnel(funnel))
}

#[put("/<id>", data = "<funnel_request>")]
pub fn update(id: i32, funnel_request: Json<UpdateFunnelRequest>) -> () {
    update_funnel(id, funnel_request.into_inner());
}

#[get("/")]
pub fn list() -> Json<Vec<FunnelResponse>> {
    let funnel_list = get_all_active_funnels();
    let response = funnel_list
        .into_iter()
        .map(|funnel| (FunnelResponse::from_funnel(funnel)))
        .collect();
    Json(response)
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Json<FunnelResponse> {
    let funnel = get_funnel_by_id(id);
    let mut funnel_response = FunnelResponse::from_funnel(funnel.expect("figure out how to handle not found"));
    let variation_list = get_all_active_variations_from_funnel_id(id);
    let variation_response_list: Vec<VariationResponse> = variation_list
        .into_iter()
        .map(|variation| (VariationResponse::from_variation(variation)))
        .collect();
    funnel_response.variations = Some(variation_response_list);
    Json(funnel_response)
}

#[delete("/<id>")]
pub fn delete(id: i32) -> () {
    soft_delete_funnel(id);
}

#[get("/<id>/variations")]
pub fn list_variations(id: i32) -> Json<Vec<VariationResponse>> {
    let variation_list = get_all_active_variations_from_funnel_id(id);
    let response = variation_list
        .into_iter()
        .map(|variation| (VariationResponse::from_variation(variation)))
        .collect();
    Json(response)
}

#[derive(Serialize)]
pub struct ABResponse {
    pub a: VariationResponse,
    pub b: VariationResponse
}
#[get("/<id>/variations/ab")]
pub fn list_ab_variations(id: i32) -> Json<ABResponse> {
    let (a, b) = get_ab_variations_by_funnel_id(id);
    Json(ABResponse { 
        a: VariationResponse::from_variation(a),
        b: VariationResponse::from_variation(b),
    })
}

#[derive(Deserialize)]
pub struct CloneRequest {
    pub variation_id: i32,
    pub funnel: NewFunnelRequest
}
#[post("/clone", data = "<clone_request>")]
pub fn clone(clone_request: Json<CloneRequest>) -> Option<Json<FunnelResponse>> {
    let new_funnel_result = create_funnel_with_cloned_variation(clone_request.variation_id, &clone_request.funnel);
    if let Some(new_funnel) = new_funnel_result {
        return Some(Json(FunnelResponse::from_funnel(new_funnel)));
    }
    None
}
