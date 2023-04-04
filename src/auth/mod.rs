mod routes;
mod requests;
mod actions;
mod models;
mod tokens;

use super::container::{ RouteBlock, Container};

use routes::*;

pub fn build() -> Container {
    Container {
        route_blocks: vec![
            RouteBlock { path: "/v1/auth".to_owned(), get_routes: get_auth_routes },
        ]
    }
}
