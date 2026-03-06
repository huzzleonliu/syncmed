-- Your SQL goes here
create table question_log (
    id serial primary key,
    x text not null,
    y text not null,
    user_answer text not null,
    correct_answer text not null,
    is_correct boolean not null,
    created_at timestamp not null default now()
);