
use super::container::Container;

mod requests;

mod response;

mod actions;

mod models;

mod routes;
use routes::routes;


pub fn build() -> Container {
    Container { path: "/v1/funnels".to_owned(), get_routes: routes }
}