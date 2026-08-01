use rand::RngExt;

pub fn generate_otp() -> String {
    let otp = rand::rng().random_range(100000..999999);

    otp.to_string()
}
