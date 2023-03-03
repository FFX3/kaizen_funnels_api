use rocket::Route;

mod funnels;
mod variations;
mod steps;

pub fn get_funnel_routes() -> Vec<Route> {
    routes![
        funnels::create,
        funnels::update,
        funnels::list,
        funnels::get_one,
        funnels::delete,
        funnels::list_variations,
        funnels::list_ab_variations
    ]
}

pub fn get_variation_routes() -> Vec<Route> {
    routes![
        variations::create,
        variations::update,
        variations::list,
        variations::delete,
        variations::mark_a,
        variations::mark_b,
        variations::mark_winner,
        variations::list_steps,
    ]
}

pub fn get_step_routes() -> Vec<Route> {
    routes![
        steps::create,
        steps::update,
        steps::list,
        steps::delete,
        steps::get_content,
        steps::get_grapesjs,
        steps::update_content,
    ]
}
