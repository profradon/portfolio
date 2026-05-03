use mongodb::{Client, Database};
use std::env;

pub async fn init() -> Result<Database, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "portfolio".to_string());

    println!("DEBUG: Connecting to MongoDB at: {}", mongo_uri);
    println!("DEBUG: Database name: {}", database_name);

    let client = Client::with_uri_str(&mongo_uri).await?;
    let database = client.database(&database_name);

    // Test the connection
    println!("DEBUG: Testing database connection...");
    let collections = database.list_collection_names().await?;
    println!("DEBUG: Found collections: {:?}", collections);

    println!("Connected to MongoDB database: {}", database_name);
    Ok(database)
}