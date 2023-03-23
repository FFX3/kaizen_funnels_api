use super::container::{ RouteBlock, Container};

mod routes;
use routes::*;

pub fn build() -> Container {
    Container {
        route_blocks: vec![
            RouteBlock { path: "/v1/media".to_owned(), get_routes: get_media_routes },
        ]
    }
}
