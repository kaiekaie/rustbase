use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

use std::env;

pub mod models;
pub mod schema;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(connection: &mut PgConnection) -> &mut PgConnection {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.

    connection.run_pending_migrations(MIGRATIONS).unwrap();
    return connection;
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    return connection;
}
