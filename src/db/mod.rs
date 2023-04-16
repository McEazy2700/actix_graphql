use dotenv::dotenv;
use dotenvy_macro::dotenv;

use sea_orm::{DatabaseConnection, Database};

pub async fn configure_db() -> Option<DatabaseConnection> {
    dotenv().ok();
    let db_user: String = dotenv!("DB_USERNAME").to_string();
    let db_password: String = dotenv!("DB_PASSWORD").to_string();
    let db_host: String = dotenv!("DB_HOST").to_string();
    let db_name: String = dotenv!("DB_NAME").to_string();
    let db_url: String = format!("postgres://{db_user}:{db_password}@{db_host}/{db_name}");

    let db: DatabaseConnection = Database::connect(db_url).await.expect("Database connection error");
    Some(db)
}
