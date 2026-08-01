use std::{println, sync::Arc};

use chrono::{Duration, Utc};

use crate::{
    models::{AppError, OtpResponse, Patient, PatientOtpRequest},
    repositories::{
        patient_otp_repository::PatientOtpRepository, patient_repository::PatientRepository,
    },
    utils::generate_otp,
};

pub struct PatientOtpService {
    otp_repository: Arc<dyn PatientOtpRepository>,
    patient_repository: Arc<dyn PatientRepository>,
}

impl PatientOtpService {
    pub fn new(
        otp_repository: Arc<dyn PatientOtpRepository>,
        patient_repository: Arc<dyn PatientRepository>,
    ) -> Self {
        Self {
            otp_repository,
            patient_repository,
        }
    }

    //*authenticating the user login */
    pub async fn request_patient_otp_service(
        &self,
        payload: PatientOtpRequest,
    ) -> Result<OtpResponse, AppError> {
        let patient = match (payload.email, payload.contact) {
            (Some(email), None) => {
                let patient = self
                    .patient_repository
                    .get_patients_by_email_trait(&email)
                    .await?;

                patient.ok_or(AppError::NotFound)
            }

            (None, Some(contact)) => {
                let patient = self
                    .patient_repository
                    .get_patients_by_contact_trait(&contact)
                    .await?;

                patient.ok_or(AppError::NotFound)
            }

            _ => Err(AppError::BadRequest(String::from(
                "Provide exactly one of email or contact number",
            ))),
        }?;

        //otp generating function
        let otp = generate_otp();

        //expiring time for the otp
        let expire_at = (Utc::now() + Duration::minutes(5)).naive_utc();

        //saving the otp to the database
        self.otp_repository
            .create_otp_trait(patient.id, &otp, expire_at)
            .await?;

        println!("successfully generated otp");

        Ok(OtpResponse {
            otp,
            expires_at: expire_at,
        })
    }
}
