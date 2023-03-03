use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Response, Request, Rocket, Build};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://127.0.0.1:3000"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS,GET"
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub fn configure_cors(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .attach(CORS)
        .mount("/", routes![all_options])
}

#[options("/<_..>")]
fn all_options() { /* Intentionally left empty */ }
