use std::sync::Arc;

use crate::config::database::Database;
use crate::constants::ORGANIZATION_MEMBER_COL_NAME;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{Document, doc};
use mongodb::{Collection, error::Result};
use shared::models::organization_member_model::OrganizationMember;

pub struct OrganizationMemberRepository {
    pub collection: Collection<OrganizationMember>,
}

impl OrganizationMemberRepository {
    pub async fn new(db: Arc<Database>) -> Result<Self> {
        let collection = db.collection(&ORGANIZATION_MEMBER_COL_NAME);
        Ok(Self { collection })
    }

    pub async fn get_all_members_by_org(
        &self,
        organization_id: &str,
    ) -> Result<Vec<OrganizationMember>> {
        let filter = doc! {
            "organization_id": organization_id,
        };
        let cursor = self.collection.find(filter).await?;
        let members: Vec<OrganizationMember> = cursor.try_collect().await?;
        Ok(members)
    }

    pub async fn create_member(&self, member: &OrganizationMember) -> Result<OrganizationMember> {
        self.collection.insert_one(member).await?;

        Ok(member.clone())
    }

    pub async fn get_all_members(&self) -> Result<Vec<OrganizationMember>> {
        let cursor = self.collection.find(doc! {}).await?;
        let members: Vec<OrganizationMember> = cursor.try_collect().await?;
        Ok(members)
    }

    pub async fn find_member(
        &self,
        field_name: &str,
        field_value: &str,
    ) -> Result<Option<OrganizationMember>> {
        let mut filter = Document::new();
        filter.insert(field_name, field_value);
        self.collection.find_one(filter).await
    }

    pub async fn find_member_by_org_and_name(
        &self,
        organization_id: &str,
        name: &str,
    ) -> Result<Option<OrganizationMember>> {
        let filter = doc! {
            "organization_id": organization_id,
            "name": name,
        };
        self.collection.find_one(filter).await
    }

    pub async fn update_member_by_org_and_name(
        &self,
        organization_id: &str,
        name: &str,
        update_doc: Document,
    ) -> Result<Option<OrganizationMember>> {
        let filter = doc! {
            "organization_id": organization_id,
            "name": name,
        };

        let mut set_doc = Document::new();
        for (key, value) in update_doc {
            if key != "_id" && key != "organization_id" {
                set_doc.insert(key, value);
            }
        }

        self.collection
            .update_one(filter.clone(), doc! { "$set": set_doc })
            .await?;

        self.find_member_by_org_and_name(organization_id, name)
            .await
    }

    pub async fn delete_member_by_org_and_name(
        &self,
        organization_id: &str,
        name: &str,
    ) -> Result<()> {
        let filter = doc! {
            "organization_id": organization_id,
            "name": name,
        };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
