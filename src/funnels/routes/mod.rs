use rocket::Route;

mod funnels;

pub fn routes() -> Vec<Route> {
    routes![
        funnels::create,
        funnels::list
    ]
}
