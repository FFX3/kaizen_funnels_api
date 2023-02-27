use diesel::PgConnection;
use diesel::prelude::*;
use rocket::Route;
use rocket::serde::{
    Deserialize,
    Serialize,
    json::Json
};
use crate::database::establish_connection;
use crate::schema::funnels;

use super::container::Container;

mod models;
use models::{
    NewFunnel,
    Funnel,
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct FunnelRequest {
    label: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct FunnelResponse {
    id: i32,
    label: String
}

#[get("/")]
fn index() -> &'static str {
    "index"
}

#[get("/<id>")]
fn find(id: &str) -> &str {
    id
}


#[post("/", data = "<funnel_request>")]
fn create(funnel_request: Json<FunnelRequest>) -> Json<FunnelResponse> {
    let connection = &mut establish_connection();
    let funnel = create_funnel(connection, funnel_request.into_inner());
    Json(FunnelResponse {
        id: funnel.id,
        label: funnel.label.to_owned()
    })
}

fn create_funnel(conn: &mut PgConnection, funnel_request: FunnelRequest) -> Funnel {
    let new_post = NewFunnel {
        label: &funnel_request.label,
        created_at: std::time::SystemTime::now()
    };
    diesel::insert_into(funnels::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new funnel")

}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        find,
        create
    ]
}

pub fn build() -> Container {
    Container { path: "/v1/funnels".to_owned(), get_routes: routes }
}