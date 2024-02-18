extern crate diesel;

use rocket_sync_db_pools::{database, diesel::SqliteConnection}; // Adjust for your database.

// Adjust the `#[database]` attribute to match your Rocket.toml configuration.
#[database("sqlite_db")] // The name here (`sqlite_db`) should match your database configuration in Rocket.toml.
pub struct DbConn(SqliteConnection);

