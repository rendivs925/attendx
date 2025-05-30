use std::sync::Arc;

use crate::config::database::Database;
use crate::constants::ORGANIZATIONS_COL_NAME;
use bson::Document;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::{Collection, error::Result};
use shared::models::organization_model::Organization;

pub struct OrganizationRepository {
    collection: Collection<Organization>,
}

impl OrganizationRepository {
    pub async fn new(db: Arc<Database>) -> Result<Self> {
        let collection = db.collection::<Organization>(&ORGANIZATIONS_COL_NAME);
        Ok(Self { collection })
    }

    pub async fn create_organization(&self, organization: Organization) -> Result<Organization> {
        self.collection.insert_one(&organization).await?;
        Ok(organization)
    }

    pub async fn find_organization_by_id(&self, org_id: &str) -> Result<Option<Organization>> {
        self.collection.find_one(doc! { "_id": org_id }).await
    }

    pub async fn find_organization(
        &self,
        field: &str,
        value: &str,
    ) -> Result<Option<Organization>> {
        let mut filter = Document::new();
        filter.insert(field, value);
        self.collection.find_one(filter).await
    }

    pub async fn get_all_organizations(&self) -> Result<Vec<Organization>> {
        let cursor = self.collection.find(doc! {}).await?;
        let organizations: Vec<Organization> = cursor.try_collect().await?;
        Ok(organizations)
    }

    pub async fn update_organization(
        &self,
        org_id: &str,
        organization: &Organization,
    ) -> Result<Organization> {
        let update_doc = to_document(organization)?;

        self.collection
            .update_one(doc! { "_id": org_id }, doc! { "$set": update_doc })
            .await?;

        Ok(organization.clone())
    }

    pub async fn delete_organization(&self, org_id: &str) -> Result<()> {
        self.collection.delete_one(doc! { "_id": org_id }).await?;
        Ok(())
    }
}
