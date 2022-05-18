use sea_orm::{Database, DatabaseConnection};

use crate::settings;

pub async fn get_connection() -> DatabaseConnection {
    let settings = settings::get_settings();
    Database::connect(&settings["DATABASE_URL"]).await.unwrap()
}
