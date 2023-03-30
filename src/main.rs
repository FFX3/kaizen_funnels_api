use rocket::Rocket;
use rocket::Build;
use std::env;
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
    let option = env::var("AUTORUN_MIGRATIONS");
    if option == Ok("true".to_string()) {
        let result = database::run_migrations();
        match result {
            Ok(_) => println!("migrations success"),
            Err(e) => println!("migration error: {e:?}"),
        }
    }
    cors::configure_cors(rocket::build())
        .mount_container(media::build())
        .mount_container(funnels::build())
}
