use rocket::{
    Rocket,
    Build,
    Route
};

pub struct RouteBlock {
    pub path: String,
    pub get_routes: fn() -> Vec<Route>,
}

pub struct Container {
    pub route_blocks: Vec<RouteBlock>
}

impl Container {
    pub fn mount_self(&self, rocket: Rocket<Build>) -> Rocket<Build> {
        self.route_blocks
            .iter()
            .fold(rocket, |rocket, block| rocket.mount(block.path.to_owned(), (block.get_routes)()))
    }
}
