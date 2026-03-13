alter table patients
    add column if not exists patient_key text;

update patients
set patient_key = md5(id::text || '|' || name || '|' || requested_at::text)
    || md5('syncmed|' || id::text || '|' || coalesce(completed_at::text, ''))
where patient_key is null or patient_key = '';

alter table patients
    alter column patient_key set not null;

create unique index if not exists idx_patients_patient_key on patients (patient_key);
