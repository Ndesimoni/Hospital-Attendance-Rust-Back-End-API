use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::{
    models::{CreatePatient, Patient, UpdatePatient},
    repository::{
        self, patient_repository::PatientRepository,
        postgres_patient_repository::PostgresPatientRepository,
    },
    services::patient_service::{
        self, create_patient_service, get_all_patients_service, get_patients_by_id_service,
        update_patient_details_service,
    },
};

////////////////////////////////////////////////////////
///

//*get all patients */
pub async fn get_all_patients(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Patient>>, StatusCode> {
    let repository = PostgresPatientRepository::new(pool);

    let patients = patient_service::get_all_patients_service(&repository)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(patients))
}

//*get patients by id*/
pub async fn get_patients_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Patient>, StatusCode> {
    println!("✅ get_patient_by_id handler called");

    let repository = PostgresPatientRepository::new(pool);

    let patient = get_patients_by_id_service(&repository, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match patient {
        Some(patient) => Ok(Json(patient)),

        None => Err(StatusCode::NOT_FOUND),
    }
}

//* create patients */
pub async fn create_patients(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePatient>,
) -> Result<Json<Patient>, StatusCode> {
    let repository = PostgresPatientRepository::new(pool);

    let patient = create_patient_service(&repository, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(patient))
}

//*update patients details*/
pub async fn update_patients_detail(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePatient>,
) -> Result<Json<Patient>, StatusCode> {
    println!("api hitting the update route handler");

    let repository = PostgresPatientRepository::new(pool);

    let patient = update_patient_details_service(&repository, id, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match patient {
        Some(p) => Ok(Json(p)),
        None => Err(StatusCode::NOT_MODIFIED),
    }
}
