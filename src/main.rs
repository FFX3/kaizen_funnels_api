use rocket::Rocket;
use rocket::Build;
mod container;
mod funnels;
mod schema;
mod database;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

trait MountsContainer {
    fn mount_container(self, container: container::Container) -> Rocket<Build>;
}

impl MountsContainer for Rocket<Build> {
    fn mount_container(self, container: container::Container) -> Rocket<Build> {
        container.mount_self(self)
    }
}

#[launch]
async fn launch() -> Rocket<Build> {
    database::establish_connection();
    rocket::build()
        .mount("/bingo", routes![index])
        .mount_container(funnels::build())
}