use std::env;
use std::path::{Path, PathBuf};
use sqlx::{MySql, MySqlPool, Pool};
use sqlx::migrate::{MigrateDatabase, MigrateError, Migrator};

pub struct Migrations {}

impl Migrations {

    pub async fn migrate() {
        let database_url: String = Self::get_db_url();
        Self::create_database_if_not_exists(database_url.as_str()).await;

        let db: Pool<MySql> = MySqlPool::connect(database_url.as_str()).await.unwrap();
        Self::run_migrations(&db).await;
    }

    fn get_db_url() -> String {
        env::var("DATABASE_URL").expect("$DATABASE_URL is not set!")
    }

    async fn create_database_if_not_exists(database_url: &str) {
        if !MySql::database_exists(database_url).await.unwrap_or(false) {
            println!("Creating database {}", database_url);
            match MySql::create_database(database_url).await {
                Ok(_) => println!("Database created successfully"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
    }

    async fn run_migrations(db: &Pool<MySql>) {
        let crate_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations: PathBuf = Path::new(&crate_dir).join("./migrations");
        let migration_results: Result<(), MigrateError> = Migrator::new(migrations)
            .await
            .unwrap()
            .run(db)
            .await;
        match migration_results {
            Ok(_) => println!("Migration success"),
            Err(error) => panic!("error: {}", error),
        }
    }

}

// async fn test(db: &Pool<MySql>) {
//     let result = sqlx::query("INSERT IGNORE INTO user (username, email, password, wallet) VALUES (?, ?, ?, ?)")
//         .bind("bobby")
//         .bind("bobby@tidyup.please")
//         .bind("aB3f9Rt2K7xPqYlO6DcX5ZvFn0sJ8jI1gHuVbW4mNzEpQyLaSrCkTiGhMwUo")
//         .bind(42.0)
//         .execute(db)
//         .await
//         .unwrap();
//     println!("Query result: {:?}", result);
//     let user_results = sqlx::query_as::<_, User>("SELECT id, username, email, password, wallet FROM user")
//         .fetch_all(db)
//         .await
//         .unwrap();
//     for user in user_results {
//         println!("[{}] name: {}, email: {}, password: {}, wallet: {}",
//                  user.id, user.username, user.email, user.password, user.wallet);
//     }
// }
