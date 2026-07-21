use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{
    models::{CreateVisit, NewVisit, UpdateVisit, Visit},
    repositories::{patient_repository::PatientRepository, visit_repository::VisitRepository},
};

pub struct PostgresVisitRepository {
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

    async fn get_all_visits_trait(&self) -> Result<Vec<Visit>, sqlx::Error> {
        let visits = sqlx::query_as!(
            Visit,
            r#"
        SELECT 
        id,
        patient_id,
        symptoms,
        diagnosis,
        medication

        FROM visits
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(visits)
    }

    async fn get_a_patient_visits_trait(&self, patient_id: i32) -> Result<Vec<Visit>, sqlx::Error> {
        let visits = sqlx::query_as!(
            Visit,
            r#"
       
        SELECT 
        id,
        patient_id,
        symptoms,
        diagnosis,
        medication
       
       FROM visits

       WHERE patient_id = $1
       "#,
            patient_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(visits)
    }

    async fn update_a_visit_trait(
        &self,
        id: i32,
        payload: UpdateVisit,
    ) -> Result<Option<Visit>, sqlx::Error> {
        let visits = sqlx::query_as!(
            Visit,
            r#"
    UPDATE visits
    SET
        symptoms = $1,
        diagnosis = $2,
        medication = $3
    WHERE id = $4
    RETURNING
        id,
        patient_id,
        symptoms,
        diagnosis,
        medication
    "#,
            &payload.symptoms,
            payload.diagnosis,
            &payload.medication,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(visits)
    }
}
