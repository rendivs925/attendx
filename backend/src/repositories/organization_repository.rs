use shared::models::organization_model::Organization;
use shared::types::requests::organization::update_organization_request::UpdateOrganizationRequest;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct OrganizationRepository {
    pub pool: PgPool,
}

impl OrganizationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_organization(&self, org: &Organization) -> Result<Organization, Error> {
        sqlx::query_as::<_, Organization>(
            "INSERT INTO organizations (id, name, email, logo_url, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(org.id)
        .bind(&org.name)
        .bind(&org.email)
        .bind(&org.logo_url)
        .bind(org.created_at)
        .bind(org.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_organization_by_id(&self, id: Uuid) -> Result<Option<Organization>, Error> {
        sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_organization_by_email(
        &self,
        email: &str,
    ) -> Result<Option<Organization>, Error> {
        sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_organizations(&self) -> Result<Vec<Organization>, Error> {
        sqlx::query_as::<_, Organization>("SELECT * FROM organizations")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update_organization(
        &self,
        id: Uuid,
        data: &UpdateOrganizationRequest,
    ) -> Result<Organization, Error> {
        sqlx::query_as::<_, Organization>(
        "UPDATE organizations SET name = $1, logo_url = $2, updated_at = now() WHERE id = $3 RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.logo_url)
    .bind(id)
    .fetch_one(&self.pool)
    .await
    }

    pub async fn delete_organization(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM organizations WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
