use rocket::Route;
use rocket::serde::json::Json;
use crate::database::establish_connection;

use crate::funnels::request::new_funnel::NewFunnelRequest;
use crate::funnels::response::funnel::FunnelResponse;
use crate::funnels::actions::create_funnel::create_funnel;

#[post("/", data = "<funnel_request>")]
pub fn create(funnel_request: Json<NewFunnelRequest>) -> Json<FunnelResponse> {
    let connection = &mut establish_connection();
    let funnel = create_funnel(connection, funnel_request.into_inner());
    Json(FunnelResponse {
        id: funnel.id,
        label: funnel.label.to_owned()
    })
}