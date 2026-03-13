create table if not exists question_log (
    id serial primary key,
    x text not null,
    y text not null,
    user_answer text not null,
    correct_answer text not null,
    is_correct boolean not null,
    created_at timestamp not null default now()
);

create table if not exists users (
    id serial primary key,
    display_name text not null,
    email text not null unique
);

create table if not exists patient (
    id serial primary key,
    patient_key text not null unique,
    doctor_user_id int4 references users(id),
    name_snapshot text not null,
    age_snapshot int4 not null check (age_snapshot >= 0 and age_snapshot <= 150),
    gender_snapshot varchar(16) not null,
    status varchar(16) not null check (status in ('sent', 'filled', 'processed')),
    requested_at timestamp(0) not null default now(),
    filled_at timestamp(0),
    constraint chk_patient_status_time_consistency check (
        (status = 'sent' and filled_at is null)
        or
        (status in ('filled', 'processed') and filled_at is not null)
    )
);

create index if not exists idx_patient_requested_at on patient (requested_at desc);
create index if not exists idx_patient_status on patient (status);

create table if not exists case_medications (
    id serial primary key,
    patient_id int4 not null references patient(id) on delete cascade,
    med_name text not null,
    dose text not null,
    frequency text not null,
    start_date date,
    end_date date,
    notes text,
    created_at timestamp(0) not null default now()
);

create index if not exists idx_case_medications_patient_id on case_medications (patient_id);

create table if not exists case_chat_messages (
    id serial primary key,
    patient_id int4 not null references patient(id) on delete cascade,
    sender_type varchar(16) not null check (sender_type in ('patient', 'bot')),
    content_text text not null,
    created_at timestamp(0) not null default now()
);

create index if not exists idx_case_chat_messages_patient_id on case_chat_messages (patient_id);

insert into patient (
    patient_key,
    doctor_user_id,
    name_snapshot,
    age_snapshot,
    gender_snapshot,
    status,
    requested_at,
    filled_at
)
values
('1f38e6d33a7b1f1ca8b63d4d07ab8a1412c4fa7f20bcf0f50b1dd8dbfdb7b6c1', null, 'Alex Wong', 30, 'female', 'filled', '2026-03-07 12:34:00', '2026-03-07 22:22:00'),
('253952bc6f9a4de2088f5641f65ef307b8fdbeab3a7fc66e8b62c8c9a95cb0c3', null, 'Lily Zhang', 25, 'male', 'sent', '2026-03-07 18:40:00', null),
('b91ef0f69210c54db9f3f94789d73cc8566945e640fb982e70d2668d53ac63f7', null, 'Nina Park', 32, 'male', 'filled', '2026-03-07 11:00:00', '2026-03-07 11:05:00'),
('5ec9dc6d456f58fb4fca5a98b8dc2f3e47e3f5094ebdb4f6465f74f2b0e69d06', null, 'Samir Khan', 28, 'female', 'sent', '2026-03-07 11:11:00', null),
('6cfd021df5287df7f6714721be39d03c0b286af92bb2f279f4f860ecf0e5a338', null, 'Emma Davis', 41, 'female', 'filled', '2026-03-08 09:22:00', '2026-03-08 09:58:00'),
('0d89a03dd8e6ff8583f6f2a12f3f06a6da9f8d8af86b97f9678d5532a703ff95', null, 'Carlos Rivera', 36, 'male', 'sent', '2026-03-08 10:47:00', null),
('7aa0f49f1a4f764a2ef5ca2935e5d2699230356288de3ad36f18276f98d12be4', null, 'Mina Choi', 29, 'female', 'filled', '2026-03-08 13:15:00', '2026-03-08 13:49:00'),
('93687f5ecbf47444a47d2f218d9418e013bde6c6d7acc5e02157f8fd6bfc9cad', null, 'Omar Hassan', 54, 'male', 'sent', '2026-03-08 16:03:00', null)
on conflict (patient_key) do nothing;

insert into case_medications (
    patient_id,
    med_name,
    dose,
    frequency,
    start_date,
    end_date,
    notes
)
select
    p.id,
    seed.med_name,
    seed.dose,
    seed.frequency,
    seed.start_date,
    seed.end_date,
    seed.notes
from patient p
join (
    values
        ('Alex Wong', 'Paracetamol', '500mg', 'twice a day', date '2026-02-01', null, 'As needed for fever'),
        ('Nina Park', 'Ibuprofen', '200mg', 'once a day', date '2026-02-10', date '2026-03-10', 'After meal')
) as seed(name_snapshot, med_name, dose, frequency, start_date, end_date, notes)
on p.name_snapshot = seed.name_snapshot
where not exists (
    select 1
    from case_medications m
    where m.patient_id = p.id
      and m.med_name = seed.med_name
);

insert into case_chat_messages (
    patient_id,
    sender_type,
    content_text,
    created_at
)
select
    p.id,
    seed.sender_type,
    seed.content_text,
    now()
from patient p
join (
    values
        ('Alex Wong', 'patient', 'I am taking Paracetamol for a recurring headache.'),
        ('Alex Wong', 'bot', 'Thank you. Please confirm your daily dosage and duration.'),
        ('Nina Park', 'patient', 'I started Ibuprofen last month and felt better.')
) as seed(name_snapshot, sender_type, content_text)
on p.name_snapshot = seed.name_snapshot
where not exists (
    select 1
    from case_chat_messages c
    where c.patient_id = p.id
      and c.sender_type = seed.sender_type
      and c.content_text = seed.content_text
);
