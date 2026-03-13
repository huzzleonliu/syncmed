use chrono::{NaiveDate, NaiveDateTime};
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
#[diesel(table_name = crate::db::schema::patient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct PatientCase {
    pub id: i32,
    pub patient_key: String,
    pub doctor_user_id: Option<i32>,
    pub name_snapshot: String,
    pub age_snapshot: i32,
    pub gender_snapshot: String,
    pub status: String,
    pub requested_at: NaiveDateTime,
    pub filled_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::patient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct NewPatientCase {
    pub patient_key: String,
    pub doctor_user_id: Option<i32>,
    pub name_snapshot: String,
    pub age_snapshot: i32,
    pub gender_snapshot: String,
    pub status: String,
    pub requested_at: NaiveDateTime,
    pub filled_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::case_medications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct CaseMedication {
    pub id: i32,
    pub patient_id: i32,
    pub med_name: String,
    pub dose: String,
    pub frequency: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::case_medications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct NewCaseMedication {
    pub patient_id: i32,
    pub med_name: String,
    pub dose: String,
    pub frequency: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::case_chat_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct CaseChatMessage {
    pub id: i32,
    pub patient_id: i32,
    pub sender_type: String,
    pub content_text: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::case_chat_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct NewCaseChatMessage {
    pub patient_id: i32,
    pub sender_type: String,
    pub content_text: String,
}
