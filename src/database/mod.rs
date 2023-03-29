use diesel::pg::PgConnection;
use diesel::Connection;
use dotenvy::dotenv;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::error::Error;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))


}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let connection = &mut establish_connection();
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
