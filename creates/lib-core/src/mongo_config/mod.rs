use mongodb::{Client, Database};

pub async fn init_mongo_client(url: &str, db_name: &str) -> Database {
    let client = Client::with_uri_str(url).await.unwrap();
    let database = client.database(db_name);
    database
}
