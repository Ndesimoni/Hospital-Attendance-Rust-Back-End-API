use std::{net::IpAddr, println, sync::Arc};

use axum::extract::ConnectInfo;
use tokio::net::unix::SocketAddr;

use crate::{
    middleware::jwt::create_token,
    models::{AppError, LoginResponse, OtpResponse, OtpVerification, PatientOtpRequest, Roles},
    repositories::patient_repository::PatientRepository,
    services::RedisService,
    utils::generate_otp,
};

pub struct PatientOtpService {
    redis_service: Arc<RedisService>,
    patient_repository: Arc<dyn PatientRepository>,
    jwt_secret: String,
}

impl PatientOtpService {
    pub fn new(
        redis_service: Arc<RedisService>,
        patient_repository: Arc<dyn PatientRepository>,
        jwt_secret: String,
    ) -> Self {
        Self {
            redis_service,
            patient_repository,
            jwt_secret,
        }
    }

    //* Request OTP
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

            _ => {
                return Err(AppError::BadRequest(
                    "Provide exactly one of email or contact number".to_string(),
                ));
            }
        }?;

        let rate_key = format!("otp_attempt:{}", patient.email);

        let allowed = self
            .redis_service
            .check_rate_limit(&rate_key, 5, 60)
            .await?;

        if !allowed {
            return Err(AppError::BadRequest(
                "Too many OTP requests. Try again later".to_string(),
            ));
        }

        // Generate OTP
        let otp = generate_otp();

        // Store OTP in Redis for 5 minutes
        self.redis_service.store_otp(patient.id, &otp).await?;

        // Simulate sending SMS
        println!("Generated OTP: {}", otp);

        Ok(OtpResponse { otp })
    }

    //* Verify OTP
    pub async fn verify_otp_service(
        &self,
        payload: OtpVerification,
        // ip_addr: IpAddr,
    ) -> Result<LoginResponse, AppError> {
        println!("========== VERIFY OTP ==========");
        println!("Email: {:?}", payload.email);
        println!("Contact: {:?}", payload.contact);
        println!("OTP: {}", payload.otp);

        // println!("Client IP: {:?}", ip_addr);

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
                    "Provide exactly one of email or contact number".to_string(),
                ));
            }
        }?;

        // Retrieve OTP from Redis
        let stored_otp = self.redis_service.get_otp(patient.id).await?;

        println!("Stored OTP: {:?}", stored_otp);

        let stored_otp = match stored_otp {
            Some(code) => code,
            None => {
                return Err(AppError::Unauthorized);
            }
        };

        // Compare OTPs
        if stored_otp != payload.otp {
            return Err(AppError::Unauthorized);
        }

        // Delete OTP after successful verification
        self.redis_service.delete_otp(patient.id).await?;

        // Generate JWT
        let token = create_token(
            patient.id,
            patient.email.clone(),
            Roles::Patient,
            &self.jwt_secret,
        )?;

        Ok(LoginResponse { token })
    }
}
