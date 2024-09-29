pub mod crud;
pub mod entity;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

pub async fn get_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").unwrap();

    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(5).min_connections(1);

    Database::connect(opt).await.unwrap()
}
