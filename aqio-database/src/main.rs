use aqio_database::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:aqio.db".to_string());

    let _db = Database::new(&database_url).await?;
    println!("Database connected successfully!");

    Ok(())
}
