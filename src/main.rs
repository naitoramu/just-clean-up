use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

const DATABASE_URL: &str = "mysql://sql7633406:Q3qfXqUHnj@sql7.freemysqlhosting.net:3306";
const DB_NAME: &str = "sql7633406";

async fn run() -> Result<(), DbErr> {
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(url).await?;
    db.execute(Statement::from_string(
        DbBackend::MySql,
        format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
    )).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
