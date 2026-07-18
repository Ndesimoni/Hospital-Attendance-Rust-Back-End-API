-- Add migration script here

CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    patient_id INT NOT NULL,
    symptoms TEXT[] NOT NULL,
    diagnosis TEXT NOT NULL,
    medication TEXT[] NOT NULL,

    CONSTRAINT fk_patient
        FOREIGN KEY (patient_id)
        REFERENCES patients(id)
);

