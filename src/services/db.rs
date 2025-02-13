use mongodb::{results::InsertOneResult, Client, Collection, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::error::Error;
use crate::models::members_model::Members;

pub struct Database {
    members: Collection<Members>,
}

const DEFAULT_MONGODB_URI: &str = "mongodb://localhost:27017";
const DB_NAME: &str = "members_db";
const MEMBERS_COLLECTION: &str = "members";
impl Database {
    pub async fn init() -> Self {
        let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| DEFAULT_MONGODB_URI.into());

        let client = Client::with_uri_str(&uri).await.expect("failed to connect");
        let db = client.database(DB_NAME);
        let members = db.collection(MEMBERS_COLLECTION);

        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(options)
            .build();

        client
            .database(DB_NAME)
            .collection::<Members>(MEMBERS_COLLECTION)
            .create_index(model)
            .await
            .expect("creating an index should succeed");

        Database { members }
    }

    pub async fn create_member(&self, member: Members) -> Result<InsertOneResult, Error> {
        let result = self.members.insert_one(member).await;

        match result {
            Ok(result) => Ok(result),
            Err(e) =>  Err(e),
        }
    }
}
