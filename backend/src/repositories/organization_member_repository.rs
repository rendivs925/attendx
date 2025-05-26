use std::sync::Arc;

use crate::config::database::Database;
use crate::constants::USER_COL_NAME;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_document};
use mongodb::{Collection, error::Result};
use shared::models::user_model::User;

pub struct OrganizationMemberRepository {
    pub collection: Collection<User>,
}

impl OrganizationMemberRepository {
    pub async fn new(db: Arc<Database>) -> Result<Self> {
        let collection = db.collection(&USER_COL_NAME);
        Ok(Self { collection })
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        self.collection.insert_one(user).await?;
        Ok(User { ..user.clone() })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let cursor = self.collection.find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    pub async fn update_user(&self, user_id: &str, user: &User) -> Result<User> {
        let object_id = ObjectId::parse_str(user_id).unwrap();
        let update_doc = to_document(user)?;

        self.collection
            .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
            .await?;

        Ok(user.clone())
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        self.collection.delete_one(doc! { "_id": user_id }).await?;
        Ok(())
    }
}
