use super::container::{ RouteBlock, Container};

mod requests;

mod responses;

mod actions;

mod models;

mod routes;
use routes::*;


pub fn build() -> Container {
    Container {
        route_blocks: vec![
            RouteBlock { path: "/v1/funnels".to_owned(), get_routes: get_funnel_routes },
            RouteBlock { path: "/v1/variations".to_owned(), get_routes: get_variation_routes },
            RouteBlock { path: "/v1/steps".to_owned(), get_routes: get_step_routes },
        ]
    }
}
