pub struct DiagnosisResult {
    pub diagnosis: String,
    pub medication: Vec<String>,
}

pub fn diagnosis_services(symptoms: &[String]) -> DiagnosisResult {
    let diagnosis =
        if symptoms.contains(&"fever".to_string()) && symptoms.contains(&"cough".to_string()) {
            "Flu".to_string()
        } else if symptoms.contains(&"headache".to_string()) {
            "Migraine".to_string()
        } else {
            "Unknown condition".to_string()
        };

    let medication = if diagnosis == "Flu" {
        vec!["Paracetamol".to_string(), "Rest".to_string()]
    } else if diagnosis == "Migraine" {
        vec!["Painkillers".to_string()]
    } else {
        vec!["Consult specialist".to_string()]
    };

    DiagnosisResult {
        diagnosis,
        medication,
    }
}
