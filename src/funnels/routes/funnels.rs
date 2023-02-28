use rocket::serde::json::Json;

use crate::funnels::models::funnel::Funnel;
use crate::funnels::requests::new_funnel::NewFunnelRequest;
use crate::funnels::responses::funnel::FunnelResponse;
use crate::funnels::actions::create_funnel::create_funnel;
use crate::funnels::actions::get_all_active_funnels::get_all_active_funnels;

trait FromFunnel {
    fn from_funnel(funnel: Funnel) -> Self;
}

impl FromFunnel for FunnelResponse {
    fn from_funnel(funnel: Funnel) -> Self {
        Self {
            id: funnel.id,
            label: funnel.label.to_owned()
        }
    }
}

#[post("/", data = "<funnel_request>")]
pub fn create(funnel_request: Json<NewFunnelRequest>) -> Json<FunnelResponse> {
    let funnel = create_funnel(funnel_request.into_inner());
    Json(FunnelResponse {
        id: funnel.id,
        label: funnel.label.to_owned()
    })
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

