use std::collections::HashMap;

pub mod auth_token {
    use rand::{distributions::Alphanumeric, Rng};
    fn generate_random_key() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(27)
            .map(char::from)
            .collect()
    }


}

pub struct AccessTokenManager {
}

pub struct AccessTokenResult {
    user_id: i32,
    permissions: HashMap<String, String>
}

impl AccessTokenManager {
    pub fn parse() -> Result<AccessTokenResult, &'static str>{
        Ok(AccessTokenResult {
            user_id: 1,
            permissions: HashMap::new()
        })
    }
}
