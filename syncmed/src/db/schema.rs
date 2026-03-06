// @generated automatically by Diesel CLI.

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
