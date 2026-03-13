drop index if exists idx_patients_patient_key;
alter table patients drop column if exists patient_key;
