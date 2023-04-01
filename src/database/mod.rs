use diesel::pg::PgConnection;
use diesel::Connection;
use dotenvy::dotenv;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::error::Error;


const TENANT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/tenant");
const GATEKEEPER_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/gatekeeper");
lazy_static! {
    static ref BASE_DATABASE_URL: String = env::var("BASE_DATABASE_URL").expect("BASE_DATABASE_URL must be set");
    static ref GATEKEEPER_DATABASE_NAME: String = env::var("GATEKEEPER_DATABASE_NAME").expect("GATEKEEPER_DATABASE_NAME must be set");
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = BASE_DATABASE_URL.to_owned() + "kaizen_funnels"; //todo should depend on gatekeeper
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_gatekeeper_connection() -> PgConnection {
    dotenv().ok();

    let database_url = BASE_DATABASE_URL.to_owned() + &GATEKEEPER_DATABASE_NAME;
    println!("url: {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_tenant_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let connection = &mut establish_connection();
    connection.run_pending_migrations(TENANT_MIGRATIONS)?;

    Ok(())
}

pub fn run_gatekeeper_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let connection = &mut establish_gatekeeper_connection();
    connection.run_pending_migrations(GATEKEEPER_MIGRATIONS)?;
    Ok(())
}
