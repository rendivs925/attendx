use crate::constants::{DB_NAME, MONGODB_URI, USER_COL_NAME};
use log::info;
use mongodb::{
    Client, Collection, IndexModel,
    bson::doc,
    error::Error as MongoError,
    options::{ClientOptions, IndexOptions},
};
use shared::models::user_model::User;

pub struct Database {
    pub client: Client,
}

impl Database {
    pub async fn new() -> Result<Self, MongoError> {
        let client_uri = (*MONGODB_URI).as_str();
        let client_options = ClientOptions::parse(client_uri).await?;
        let client = Client::with_options(client_options)?;
        info!("Connected to MongoDB");

        let db = Self { client };
        db.create_unique_indexes().await?;
        Ok(db)
    }

    pub fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: serde::de::DeserializeOwned + serde::Serialize + Unpin + Send + Sync,
    {
        self.client.database(&DB_NAME).collection::<T>(name)
    }

    async fn create_partial_unique_index(
        &self,
        collection: &Collection<User>,
        field: &str,
    ) -> Result<(), MongoError> {
        let index = IndexModel::builder()
            .keys(doc! { field: 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .partial_filter_expression(
                        doc! { field: { "$exists": true, "$type": "string" } },
                    )
                    .build(),
            )
            .build();

        collection.create_index(index).await?;
        Ok(())
    }

    async fn create_unique_indexes(&self) -> Result<(), MongoError> {
        let collection = self.collection::<User>(&USER_COL_NAME);

        self.create_partial_unique_index(&collection, "email")
            .await?;
        self.create_partial_unique_index(&collection, "username")
            .await?;
        self.create_partial_unique_index(&collection, "nim").await?;
        self.create_partial_unique_index(&collection, "nidn")
            .await?;

        Ok(())
    }
}
