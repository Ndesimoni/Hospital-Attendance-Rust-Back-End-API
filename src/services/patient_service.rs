use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{
    models::{CreatePatient, Patient, Visit},
    // state::DB,
};

// pub fn create_patient_service(db: DB, payload: CreatePatient) -> Result<Patient, StatusCode> {
//     let mut state = db.lock().unwrap();

//     // 1. CHECK DUPLICATE
//     let exists = state
//         .patients
//         .iter()
//         .any(|p| p.name == payload.name && p.age == payload.age && p.gender == payload.gender);

//     if exists {
//         return Err(StatusCode::CONFLICT);
//     }

//     // 2. CREATE NEW PATIENT
//     let new_patient = Patient {
//         id: (state.patients.len() as u32) + 1,
//         name: payload.name,
//         age: payload.age,
//         gender: payload.gender,
//     };

//     state.patients.push(new_patient.clone());

//     Ok(new_patient)
// }

// pub fn get_all_patients_service(db: DB) -> Vec<Patient> {
//     let state = db.lock().unwrap();
//     state.patients.clone()
// }

// //todo fine patient by params id
// pub fn find_patient_service(db: DB, id: u32) -> Result<Patient, StatusCode> {
//     let state = db.lock().unwrap();

//     let patient = state.patients.iter().find(|p| p.id == id).cloned();

//     match patient {
//         Some(p) => Ok(p),
//         None => Err(StatusCode::NOT_FOUND),
//     }
// }

// pub fn update_patient_service(
//     db: DB,
//     id: u32,
//     payload: CreatePatient,
// ) -> Result<Patient, StatusCode> {
//     let mut state = db.lock().unwrap();

//     let patient = state.patients.iter_mut().find(|p| p.id == id);

//     match patient {
//         Some(p) => {
//             p.name = payload.name;
//             p.age = payload.age;
//             p.gender = payload.gender;

//             Ok(p.clone())
//         }

//         None => Err(StatusCode::NOT_FOUND),
//     }
// }

// pub fn delete_patient_service(db: DB, id: u32) -> StatusCode {
//     let mut state = db.lock().unwrap();

//     let before = state.patients.len();

//     state.patients.retain(|p| p.id != id);

//     let after = state.patients.len();

//     if before == after {
//         return StatusCode::NOT_FOUND;
//     } else {
//         return StatusCode::NO_CONTENT;
//     }
// }

pub async fn create_patient_service(
    pool: &PgPool,
    payload: CreatePatient,
) -> Result<Patient, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        r#"
      INSERT INTO patients(
        name,age,gender,email,contact
      ) VALUES(
        $1,$2, $3, $4, $5
      )RETURNING id,name,age,gender,email,contact"#,
        payload.name,
        payload.age,
        payload.gender,
        payload.email,
        payload.contact
    )
    .fetch_one(pool)
    .await?;

    Ok(patient)
}

pub async fn get_all_patients_service(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error> {
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
    .fetch_all(pool)
    .await?;

    Ok(patient)
}
