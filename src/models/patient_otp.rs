use chrono::NaiveDateTime;
use serde::Serialize;

pub struct PatientOtps {
    pub id: i32,
    pub patient_id: i32,
    pub otp: String,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub create_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct OtpResponse {
    pub otp: String,
    pub expires_at: NaiveDateTime,
}
