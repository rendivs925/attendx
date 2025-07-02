use shared::models::attendance_model::Attendance;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct AttendanceRepository {
    pub pool: PgPool,
}

impl AttendanceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_attendance(&self, a: &Attendance) -> Result<Attendance, Error> {
        sqlx::query_as::<_, Attendance>(
            r#"
            INSERT INTO attendances (
                id, user_id, organization_id,
                clock_in, clock_out, date,
                method, status, attendance_type,
                lat, long, created_at, updated_at
            ) VALUES (
                $1, $2, $3,
                $4, $5, $6,
                $7, $8, $9,
                $10, $11, $12, $13
            ) RETURNING *
        "#,
        )
        .bind(a.id)
        .bind(a.user_id)
        .bind(a.organization_id)
        .bind(a.clock_in)
        .bind(a.clock_out)
        .bind(a.date)
        .bind(a.method.clone())
        .bind(a.status.clone())
        .bind(a.attendance_type.clone())
        .bind(a.lat)
        .bind(a.long)
        .bind(a.created_at)
        .bind(a.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_attendance_by_id(&self, id: Uuid) -> Result<Option<Attendance>, Error> {
        sqlx::query_as::<_, Attendance>("SELECT * FROM attendances WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_attendances(&self) -> Result<Vec<Attendance>, Error> {
        sqlx::query_as::<_, Attendance>("SELECT * FROM attendances")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_all_attendances_for_user_in_org(
        &self,
        user_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<Attendance>, Error> {
        sqlx::query_as::<_, Attendance>(
            "SELECT * FROM attendances WHERE user_id = $1 AND organization_id = $2",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_all_attendances_for_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<Attendance>, Error> {
        sqlx::query_as::<_, Attendance>("SELECT * FROM attendances WHERE organization_id = $1")
            .bind(org_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update_attendance(&self, id: Uuid, a: &Attendance) -> Result<Attendance, Error> {
        sqlx::query_as::<_, Attendance>(
            r#"
        UPDATE attendances SET
            clock_in = $1, clock_out = $2, date = $3,
            method = $4, status = $5, attendance_type = $6,
            lat = $7, long = $8, updated_at = $9
        WHERE id = $10
        RETURNING *
    "#,
        )
        .bind(a.clock_in)
        .bind(a.clock_out)
        .bind(a.date)
        .bind(a.method.clone())
        .bind(a.status.clone())
        .bind(a.attendance_type.clone())
        .bind(a.lat)
        .bind(a.long)
        .bind(a.updated_at)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_attendance(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM attendances WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
