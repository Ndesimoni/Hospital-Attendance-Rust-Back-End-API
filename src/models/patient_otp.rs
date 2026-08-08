use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

//* otp request body struct*/
#[derive(Debug, PartialEq, Eq)]
pub struct PatientOtps {
    pub id: i32,
    pub patient_id: i32,
    pub otp: String,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub created_at: NaiveDateTime,
}

//* this is the patient login struct */
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PatientOtpRequest {
    #[validate(email)]
    pub email: Option<String>,
    pub contact: Option<String>,
}

//* otp response body struct*/
#[derive(Debug, Serialize)]
pub struct OtpResponse {
    pub otp: String,
}

//* otp verification body struct*/
#[derive(Debug, Deserialize)]
pub struct OtpVerification {
    pub email: Option<String>,
    pub contact: Option<String>,
    pub otp: String,
}
