use std::{println, sync::Arc};

use chrono::{Duration, Utc};

use crate::{
    middleware::jwt::create_token,
    models::{AppError, LoginResponse, OtpResponse, OtpVerification, PatientOtpRequest, Roles},
    repositories::{
        patient_otp_repository::PatientOtpRepository, patient_repository::PatientRepository,
    },
    utils::generate_otp,
};

pub struct PatientOtpService {
    otp_repository: Arc<dyn PatientOtpRepository>,
    patient_repository: Arc<dyn PatientRepository>,
    jwt_secret: String,
}

impl PatientOtpService {
    pub fn new(
        otp_repository: Arc<dyn PatientOtpRepository>,
        patient_repository: Arc<dyn PatientRepository>,
        jwt_secret: String,
    ) -> Self {
        Self {
            otp_repository,
            patient_repository,
            jwt_secret,
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

        // Delete old OTP first
        self.otp_repository
            .delete_by_patient_id_trait(patient.id)
            .await?;

        // Generate new OTP
        let otp = generate_otp();

        let expire_at = (Utc::now() + Duration::minutes(5)).naive_utc();

        // Save new OTP
        self.otp_repository
            .create_otp_trait(patient.id, &otp, expire_at)
            .await?;

        println!("successfully generated otp");

        Ok(OtpResponse {
            otp,
            expires_at: expire_at,
        })
    }

    pub async fn verify_otp_service(
        &self,
        payload: OtpVerification,
    ) -> Result<LoginResponse, AppError> {
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

            _ => {
                return Err(AppError::BadRequest(
                    "Provide either email or contact".into(),
                ));
            }
        }?;

        let patient_otp = self
            .otp_repository
            .find_by_patient_id_trait(patient.id)
            .await?;

        // Check OTP expiry
        if patient_otp.expires_at < Utc::now().naive_utc() {
            return Err(AppError::Unauthorized);
        }

        // Check OTP value
        if patient_otp.otp != payload.otp {
            return Err(AppError::Unauthorized);
        }

        // Remove OTP after successful verification
        self.otp_repository
            .delete_by_patient_id_trait(patient.id)
            .await?;

        // Create patient JWT
        let token = create_token(
            patient.id,
            patient.email.clone(),
            Roles::Patient,
            &self.jwt_secret,
        )?;

        Ok(LoginResponse { token })
    }
}
