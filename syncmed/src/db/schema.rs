// @generated automatically by Diesel CLI.

diesel::table! {
    patients (id) {
        id -> Int4,
        name -> Text,
        age -> Int4,
        #[max_length = 16]
        gender -> Varchar,
        #[max_length = 32]
        report_status -> Varchar,
        requested_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        patient_key -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(patients, question_log,);
