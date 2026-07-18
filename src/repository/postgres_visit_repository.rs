use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    models::{CreateVisit, NewVisit, Visit},
    repository::{patient_repository::PatientRepository, visit_repository::VisitRepository},
};

struct PostgresVisitRepository {
    pool: PgPool,
}

impl PostgresVisitRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VisitRepository for PostgresVisitRepository {
    async fn create_visit_trait(&self, payload: NewVisit) -> Result<Visit, sqlx::Error> {
        let create_visit = sqlx::query_as!(
            Visit,
            r#"
    INSERT INTO visits( 
    patient_id,
    symptoms,
    diagnosis,
    medication)

    VALUES ($1,$2, $3, $4 )

    RETURNING id, patient_id, symptoms, diagnosis, medication;
    "#,
            payload.patient_id,
            &payload.symptoms,
            payload.diagnosis,
            &payload.medication
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(create_visit)
    }
}
