use dotenv::dotenv;
use std::env;

// src/setup.rs

use sea_orm::*;

// Replace with your database URL and database name

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let database_url: std::string::String = env::var("DATABASE_URL").unwrap();
    let database_name: std::string::String = env::var("DATABASE_NAME").unwrap();
    let db = Database::connect(database_url.to_string()).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", database_name),
            ))
            .await?;

            let url = format!("{}/{}", database_url, database_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", database_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", database_name),
            ))
            .await?;

            let url = format!("{}/{}", database_url, database_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}
