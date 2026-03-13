-- 患者主表：用于 patient-url 页面存储患者请求与处理状态
create table patients (
    id serial primary key,
    patient_key text not null,
    name text not null,
    age int4 not null check (age >= 0 and age <= 150),
    gender varchar(16) not null,
    report_status varchar(32) not null,
    requested_at timestamp(0) not null default now(),
    completed_at timestamp(0)
);

create index idx_patients_requested_at on patients (requested_at desc);
create index idx_patients_report_status on patients (report_status);

-- 初始化 browse_patient_entry 页面当前展示的数据
insert into patients (patient_key, name, age, gender, report_status, requested_at, completed_at) values
('1f38e6d33a7b1f1ca8b63d4d07ab8a1412c4fa7f20bcf0f50b1dd8dbfdb7b6c1', 'Alex Wong', 30, 'female', 'finished', '2026-03-07 12:34:00', '2026-03-07 22:22:00'),
('253952bc6f9a4de2088f5641f65ef307b8fdbeab3a7fc66e8b62c8c9a95cb0c3', 'Lily Zhang', 25, 'male', 'request sent', '2026-03-07 18:40:00', null),
('b91ef0f69210c54db9f3f94789d73cc8566945e640fb982e70d2668d53ac63f7', 'Nina Park', 32, 'male', 'finished', '2026-03-07 11:00:00', '2026-03-07 11:05:00'),
('5ec9dc6d456f58fb4fca5a98b8dc2f3e47e3f5094ebdb4f6465f74f2b0e69d06', 'Samir Khan', 28, 'female', 'request sent', '2026-03-07 11:11:00', null),
('6cfd021df5287df7f6714721be39d03c0b286af92bb2f279f4f860ecf0e5a338', 'Emma Davis', 41, 'female', 'finished', '2026-03-08 09:22:00', '2026-03-08 09:58:00'),
('0d89a03dd8e6ff8583f6f2a12f3f06a6da9f8d8af86b97f9678d5532a703ff95', 'Carlos Rivera', 36, 'male', 'request sent', '2026-03-08 10:47:00', null),
('7aa0f49f1a4f764a2ef5ca2935e5d2699230356288de3ad36f18276f98d12be4', 'Mina Choi', 29, 'female', 'finished', '2026-03-08 13:15:00', '2026-03-08 13:49:00'),
('93687f5ecbf47444a47d2f218d9418e013bde6c6d7acc5e02157f8fd6bfc9cad', 'Omar Hassan', 54, 'male', 'request sent', '2026-03-08 16:03:00', null);
