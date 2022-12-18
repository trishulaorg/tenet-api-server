use sea_orm::*;

pub async fn set_up_db(database_url: &str, database_name: &str) -> Result<DatabaseConnection, DbErr> {
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


pub async fn db_connection(database_url: &str, database_name: &str) -> DatabaseConnection {
    match set_up_db(database_url, database_name).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err), // WIP: should be replaced with some retries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;
    #[tokio::test]
    async fn it_works() {
        dotenv().ok();
        let database_url: std::string::String = env::var("DATABASE_URL").unwrap();
        let database_name: std::string::String = env::var("DATABASE_NAME").unwrap();
        db_connection(&database_url, &database_name).await;
        
    }
    #[tokio::test]
    #[should_panic]
    async fn it_fails_on_invalid_url() {
        let database_url = "test";
        let database_name = "test";
        db_connection(&database_url, &database_name).await;

    }
}
