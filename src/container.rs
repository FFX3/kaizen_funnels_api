use rocket::{
    Rocket,
    Build,
    Route
};

pub struct Container {
    pub path: String,
    pub get_routes: fn() -> Vec<Route>,
}

impl Container {
    pub fn mount_self(&self, rocket: Rocket<Build>) -> Rocket<Build> {
        rocket.mount(&self.path, (&self.get_routes)())
    }
}