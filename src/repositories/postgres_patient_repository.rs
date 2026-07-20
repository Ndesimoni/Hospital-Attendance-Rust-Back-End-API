use crate::{
    models::{CreatePatient, Patient, UpdatePatient},
    repositories::patient_repository::PatientRepository,
};
use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::PgPool;

//////////////////////////////////
///

pub struct PostgresPatientRepository {
    pool: PgPool,
}

impl PostgresPatientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PatientRepository for PostgresPatientRepository {
    async fn get_all_patients_trait(&self) -> Result<Vec<Patient>, sqlx::Error> {
        let patient = sqlx::query_as!(
            Patient,
            r#"
      SELECT
      id,
      name,
      age,
      gender,
      email,
      contact

      FROM patients"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(patient)
    }

    //get patient repo
    async fn get_patients_by_id_trait(&self, id: i32) -> Result<Option<Patient>, sqlx::Error> {
        let patient = sqlx::query_as!(
            Patient,
            r#"
        SELECT
            id,
            name,
            age,
            gender,
            email,
            contact

        FROM patients

        WHERE id = $1
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(patient)
    }

    async fn create_patients_trait(&self, payload: CreatePatient) -> Result<Patient, sqlx::Error> {
        let patients = sqlx::query_as!(
            Patient,
            r#"
        INSERT INTO patients (
            name,
            age,
            gender,
            email,
            contact
        )
        VALUES($1, $2, $3, $4, $5)

        RETURNING
            id,
            name,
            age,
            gender,
            email,
            contact
        "#,
            payload.name,
            payload.age,
            payload.gender,
            payload.email,
            payload.contact
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(patients)
    }

    async fn update_patients_detail_trait(
        &self,
        id: i32,
        payload: UpdatePatient,
    ) -> Result<Option<Patient>, sqlx::Error> {
        let patient = sqlx::query_as!(
            Patient,
            r#"
        UPDATE patients
        SET
            email = $1,
            contact = $2

        WHERE id = $3

        RETURNING
            id,
            name,
            age,
            gender,
            email,
            contact
        "#,
            payload.email,
            payload.contact,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(patient)
    }
}
