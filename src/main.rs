use rocket::Rocket;
use rocket::Build;
use std::env;
mod container;
mod funnels;
mod media;
mod schema;
mod database;
mod cors;
use dotenvy::dotenv;

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

trait MountsContainer {
    fn mount_container(self, container: container::Container) -> Rocket<Build>;
}

impl MountsContainer for Rocket<Build> {
    fn mount_container(self, container: container::Container) -> Rocket<Build> {
        container.mount_self(self)
    }
}

fn auto_run_gatekeeper_migrations() {
    dotenv().ok();
    let option = env::var("AUTORUN_GATEKEEPER_MIGRATIONS");
    if option == Ok("true".to_string()) {
        let result = database::run_gatekeeper_migrations();
        match result {
            Ok(_) => println!("migrations success"),
            Err(e) => println!("migration error: {e:?}"),
        }
    }
}

#[launch]
async fn launch() -> Rocket<Build> {
    auto_run_gatekeeper_migrations();
    cors::configure_cors(rocket::build())
        .mount_container(media::build())
        .mount_container(funnels::build())
}
