mod entities;

use entities::user::User;
use sqlx::{migrate::MigrateDatabase, MySql, MySqlPool, Pool};

const DATABASE_URL: &str = "mysql://sql7633406:Q3qfXqUHnj@sql7.freemysqlhosting.net:3306/sql7633406";

#[async_std::main]
async fn main() {
    create_database_if_not_exists().await;
    let db = MySqlPool::connect(DATABASE_URL).await.unwrap();
    create_tables_if_not_exists(&db).await;
    test(&db).await
}

async fn create_database_if_not_exists() {
    if !MySql::database_exists(DATABASE_URL).await.unwrap_or(false) {
        println!("Creating database {}", DATABASE_URL);
        match MySql::create_database(DATABASE_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

async fn create_tables_if_not_exists(db: &Pool<MySql>) {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);
}

async fn test(db: &Pool<MySql>) {
    let result = sqlx::query("INSERT IGNORE INTO user (username, email, password, wallet) VALUES (?, ?, ?, ?)")
        .bind("bobby")
        .bind("bobby@tidyup.please")
        .bind("aB3f9Rt2K7xPqYlO6DcX5ZvFn0sJ8jI1gHuVbW4mNzEpQyLaSrCkTiGhMwUo")
        .bind(42.0)
        .execute(db)
        .await
        .unwrap();
    println!("Query result: {:?}", result);
    let user_results = sqlx::query_as::<_, User>("SELECT id, username, email, password, wallet FROM user")
        .fetch_all(db)
        .await
        .unwrap();
    for user in user_results {
        println!("[{}] name: {}, email: {}, password: {}, wallet: {}",
                 user.id, user.username, user.email, user.password, user.wallet);
    }
}
