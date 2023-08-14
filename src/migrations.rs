use std::env;
use std::path::{Path, PathBuf};
use sqlx::{MySql,Pool};
use sqlx::migrate::{MigrateError, Migrator};

pub struct Migrations {}

impl Migrations {

    pub async fn run_migrations(db: &Pool<MySql>) {
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
