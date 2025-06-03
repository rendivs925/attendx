use std::sync::Arc;

use crate::config::database::Database;
use crate::constants::ATTENDANCE_COL_NAME;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{Document, doc, oid::ObjectId};
use mongodb::{Collection, error::Result};
use shared::models::attendance_model::Attendance;

pub struct AttendanceRepository {
    pub collection: Collection<Attendance>,
}

impl AttendanceRepository {
    pub async fn new(db: Arc<Database>) -> Result<Self> {
        let collection = db.collection(&ATTENDANCE_COL_NAME);
        Ok(Self { collection })
    }

    pub async fn create_attendance(&self, attendance: &Attendance) -> Result<Attendance> {
        self.collection.insert_one(attendance).await?;
        Ok(attendance.clone())
    }

    pub async fn get_attendance_by_id(
        &self,
        attendance_id: &ObjectId,
    ) -> Result<Option<Attendance>> {
        let filter = doc! { "_id": attendance_id };
        self.collection.find_one(filter).await
    }

    pub async fn get_all_attendances_for_user_in_org(
        &self,
        user_id: &ObjectId,
        organization_id: &ObjectId,
    ) -> Result<Vec<Attendance>> {
        let filter = doc! {
            "user_id": user_id,
            "organization_id": organization_id,
        };
        let cursor = self.collection.find(filter).await?;
        cursor.try_collect().await
    }

    pub async fn get_all_attendances_for_org(
        &self,
        organization_id: &ObjectId,
    ) -> Result<Vec<Attendance>> {
        let filter = doc! { "organization_id": organization_id };
        let cursor = self.collection.find(filter).await?;
        cursor.try_collect().await
    }

    pub async fn update_attendance(
        &self,
        attendance_id: &ObjectId,
        update_doc: Document,
    ) -> Result<Option<Attendance>> {
        let filter = doc! { "_id": attendance_id };

        let mut set_doc = Document::new();
        for (key, value) in update_doc {
            if key != "_id" && key != "user_id" && key != "organization_id" {
                set_doc.insert(key, value);
            }
        }

        if set_doc.is_empty() {
            return self.get_attendance_by_id(attendance_id).await;
        }

        self.collection
            .update_one(filter.clone(), doc! { "$set": set_doc })
            .await?;

        self.get_attendance_by_id(attendance_id).await
    }

    pub async fn delete_attendance(&self, attendance_id: &ObjectId) -> Result<()> {
        let filter = doc! { "_id": attendance_id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
