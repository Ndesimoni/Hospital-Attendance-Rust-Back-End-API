use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientOtpRequest {
    pub email_otp: String,
    pub contact_otp: String,
}
