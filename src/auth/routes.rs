use rocket::Route;

pub fn get_auth_routes() -> Vec<Route> {
    routes![
        test,
    ]
}

#[get("/test")]
async fn test() -> () {
    ()
}
