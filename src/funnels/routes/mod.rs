use rocket::Route;

mod funnels;
mod variations;

pub fn get_funnel_routes() -> Vec<Route> {
    routes![
        funnels::create,
        funnels::list,
        funnels::delete,
        funnels::list_variations
    ]
}

pub fn get_variation_routes() -> Vec<Route> {
    routes![
        variations::create,
        variations::list
    ]
}
