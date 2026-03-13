use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::question_log)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct QuestionLog {
    pub id: i32,
    pub x: String,
    pub y: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub is_correct: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::question_log)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct NewQuestionLog {
    pub x: String,
    pub y: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub is_correct: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::patients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: String,
    pub report_status: String,
    pub requested_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub patient_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::patients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct NewPatient {
    pub patient_key: String,
    pub name: String,
    pub age: i32,
    pub gender: String,
    pub report_status: String,
    pub requested_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}
