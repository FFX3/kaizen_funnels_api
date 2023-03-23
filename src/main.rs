use rocket::Rocket;
use rocket::Build;
mod container;
mod funnels;
mod media;
mod schema;
mod database;
mod cors;

#[macro_use] extern crate rocket;

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
    cors::configure_cors(rocket::build())
        .mount_container(media::build())
        .mount_container(funnels::build())
}
