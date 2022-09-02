
use log::debug;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, ConnectionTrait, DbBackend, Statement, DatabaseConnection, DbErr};

const DATABASE_URL: &str = "DATABASE_URL";
const DATABASE_NAME: &str = "DATABASE_NAME";

pub async fn db_connect() -> Result<DatabaseConnection, DbErr> {
    let db_url = std::env::var(DATABASE_URL)
       .expect("DATABASE_URL enviroment variable should be set");
    let db_name = std::env::var(DATABASE_NAME)
       .expect("DATABASE_NAME enviroment variable should be set");
    
    let db = Database::connect(&db_url).await?;
    let db = match db.get_database_backend() {
        DbBackend::Postgres => {
            let create_statement = Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            );
            
            if db.execute(create_statement).await.is_ok() {
                debug!("DB: created database");
            }

            let url = format!("{}/{}", db_url, db_name);
            Database::connect(&url).await?
        },

        _ => panic!("The database backend is not supported"),
    };

    Migrator::up(&db, None).await?;

    Ok(db)
}