-- Add migration script here
--we are adding a role to colon to the patients table

ALTER TABLE patients
ADD COLUMN role TEXT NOT NULL DEFAULT 'Patient';