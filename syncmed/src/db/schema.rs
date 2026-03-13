// @generated automatically by Diesel CLI.

diesel::table! {
    case_chat_messages (id) {
        id -> Int4,
        patient_id -> Int4,
        #[max_length = 16]
        sender_type -> Varchar,
        content_text -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    case_medications (id) {
        id -> Int4,
        patient_id -> Int4,
        med_name -> Text,
        dose -> Text,
        frequency -> Text,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    patient (id) {
        id -> Int4,
        patient_key -> Text,
        doctor_user_id -> Nullable<Int4>,
        name_snapshot -> Text,
        age_snapshot -> Int4,
        #[max_length = 16]
        gender_snapshot -> Varchar,
        #[max_length = 16]
        status -> Varchar,
        requested_at -> Timestamp,
        filled_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    question_log (id) {
        id -> Int4,
        x -> Text,
        y -> Text,
        user_answer -> Text,
        correct_answer -> Text,
        is_correct -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        display_name -> Text,
        email -> Text,
    }
}

diesel::joinable!(case_chat_messages -> patient (patient_id));
diesel::joinable!(case_medications -> patient (patient_id));
diesel::joinable!(patient -> users (doctor_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    case_chat_messages,
    case_medications,
    patient,
    question_log,
    users,
);
