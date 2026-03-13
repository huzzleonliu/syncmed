alter table patients drop constraint if exists chk_patients_status_time_consistency;
alter table patients drop constraint if exists chk_patients_report_status;
