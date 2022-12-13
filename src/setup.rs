use std::env;
use dotenv::dotenv;

// src/setup.rs

use sea_orm::*;

// Replace with your database URL and database name

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let DATABASE_URL: std::string::String = env::var("DATABASE_URL").unwrap();
    let DATABASE_NAME: std::string::String = env::var("DATABASE_NAME").unwrap();
    let db = Database::connect(DATABASE_URL.to_string()).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DATABASE_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DATABASE_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DATABASE_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DATABASE_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DATABASE_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}