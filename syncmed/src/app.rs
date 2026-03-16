use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::{
    StaticSegment,
    components::{ParentRoute, Route, Router, Routes},
};

use crate::pages::app::{
    AppChatAccessibilityPage, AppChatConfirmPage, AppChatDefaultPage, AppConfirmIdentityPage,
    AppConfirmSuccessPage, AppInputPage, AppLoginPage,
};
use crate::structure::chat::{CaseChatMessageDraft, CaseMedicationPayload};
use crate::pages::windows::{
    WindowsBrowsePatientEntryPage, WindowsCardDetailsPage, WindowsGenerateUrlPage, WindowsLayout,
    WindowsLoginPage, WindowsWidgetPage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/leptos_tailwind.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(RwSignal::new(Vec::<CaseMedicationPayload>::new()));
    provide_context(RwSignal::new(Vec::<CaseChatMessageDraft>::new()));

    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
                <Route path=(StaticSegment("app"), StaticSegment("login")) view=AppLoginPage/>
                <Route path=(StaticSegment("app"), StaticSegment("input-page")) view=AppInputPage/>
                <Route path=(StaticSegment("app"), StaticSegment("confirm-identity")) view=AppConfirmIdentityPage/>
                <Route path=(StaticSegment("app"), StaticSegment("chat-default")) view=AppChatDefaultPage/>
                <Route path=(StaticSegment("app"), StaticSegment("chat-accessibility")) view=AppChatAccessibilityPage/>
                <Route path=(StaticSegment("app"), StaticSegment("chat-confirm")) view=AppChatConfirmPage/>
                <Route path=(StaticSegment("app"), StaticSegment("confirm-success-page")) view=AppConfirmSuccessPage/>
                <Route path=StaticSegment("widget") view=WindowsWidgetPage/>
                <ParentRoute path=StaticSegment("windows") view=WindowsLayout>
                    <Route path=StaticSegment("login") view=WindowsLoginPage/>
                    <Route path=StaticSegment("generate-url") view=WindowsGenerateUrlPage/>
                    <Route path=StaticSegment("browse-patient-entry") view=WindowsBrowsePatientEntryPage/>
                    <Route path=StaticSegment("card-details") view=WindowsCardDetailsPage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <div class="min-h-screen bg-base-100 text-base-content">
            <div class="mx-auto flex w-full max-w-3xl flex-col gap-4 px-6 py-10">
                <h1 class="text-3xl font-bold">"SyncMed Routes"</h1>
                <A href="/app/login" attr:class="link">"/app/login"</A>
                <A href="/app/input-page" attr:class="link">"/app/input-page"</A>
                <A href="/app/confirm-identity" attr:class="link">"/app/confirm-identity"</A>
                <A href="/app/chat-default" attr:class="link">"/app/chat-default"</A>
                <A href="/app/chat-accessibility" attr:class="link">"/app/chat-accessibility"</A>
                <A href="/app/chat-confirm" attr:class="link">"/app/chat-confirm"</A>
                <A href="/app/confirm-success-page" attr:class="link">"/app/confirm-success-page"</A>
                <A href="/medical-system" attr:class="link">"/medical-system"</A>
                <A href="/widget" attr:class="link">"/widget"</A>
                <A href="/windows/login" attr:class="link">"/windows/login"</A>
                <A href="/windows/generate-url" attr:class="link">"/windows/generate-url"</A>
                <A href="/windows/browse-patient-entry" attr:class="link">"/windows/browse-patient-entry"</A>
                <A href="/windows/card-details" attr:class="link">"/windows/card-details"</A>
                // 添加刷新数据库的按钮
                <button type="button" class="btn btn-primary" on:click=move |_| {
                    leptos::task::spawn_local(async move {
                        let _ = refresh_database().await;
                    });
                }>
                    "Refresh Database"
                </button>
            </div>
        </div>
    }
}

#[server(RefreshDatabase, "/api")]
pub async fn refresh_database() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::DbPool;
        use axum::Extension;
        use diesel::sql_query;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;

        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;

        sql_query("truncate table question_log, case_chat_messages, case_medications, patient, users restart identity cascade;")
            .execute(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("truncate failed: {e}")))?;

        sql_query(
            "insert into users (display_name, role, email) values
            ('Iris Bennett', 'nurse', 'iris.bennett@syncmed.no'),
            ('Owen Clarke', 'doctor', 'owen.clarke@syncmed.no'),
            ('Maya Lind', 'nurse', 'maya.lind@syncmed.no'),
            ('Lucas Berg', 'doctor', 'lucas.berg@syncmed.no');",
        )
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("insert users failed: {e}")))?;

        sql_query(
            "insert into patient (
                patient_key, doctor_user_id, name_snapshot, age_snapshot, gender_snapshot, status, requested_at, filled_at, modified_at
            ) values
            ('1f38e6d33a7b1f1ca8b63d4d07ab8a1412c4fa7f20bcf0f50b1dd8dbfdb7b6c1', (select id from users where email = 'owen.clarke@syncmed.no'), 'Alex Wong', 30, 'female', 'filled', '2026-03-07 12:34:00', '2026-03-07 22:22:00', '2026-03-07 22:37:00'),
            ('253952bc6f9a4de2088f5641f65ef307b8fdbeab3a7fc66e8b62c8c9a95cb0c3', (select id from users where email = 'lucas.berg@syncmed.no'), 'Lily Zhang', 25, 'male', 'sent', '2026-03-07 18:40:00', null, '2026-03-07 18:53:00'),
            ('b91ef0f69210c54db9f3f94789d73cc8566945e640fb982e70d2668d53ac63f7', (select id from users where email = 'owen.clarke@syncmed.no'), 'Nina Park', 32, 'male', 'filled', '2026-03-07 11:00:00', '2026-03-07 11:05:00', '2026-03-07 11:19:00'),
            ('5ec9dc6d456f58fb4fca5a98b8dc2f3e47e3f5094ebdb4f6465f74f2b0e69d06', (select id from users where email = 'lucas.berg@syncmed.no'), 'Samir Khan', 28, 'female', 'sent', '2026-03-07 11:11:00', null, '2026-03-07 11:27:00'),
            ('6cfd021df5287df7f6714721be39d03c0b286af92bb2f279f4f860ecf0e5a338', (select id from users where email = 'owen.clarke@syncmed.no'), 'Emma Davis', 41, 'female', 'filled', '2026-03-08 09:22:00', '2026-03-08 09:58:00', '2026-03-08 10:11:00'),
            ('0d89a03dd8e6ff8583f6f2a12f3f06a6da9f8d8af86b97f9678d5532a703ff95', (select id from users where email = 'lucas.berg@syncmed.no'), 'Carlos Rivera', 36, 'male', 'sent', '2026-03-08 10:47:00', null, '2026-03-08 11:02:00'),
            ('7aa0f49f1a4f764a2ef5ca2935e5d2699230356288de3ad36f18276f98d12be4', (select id from users where email = 'owen.clarke@syncmed.no'), 'Mina Choi', 29, 'female', 'filled', '2026-03-08 13:15:00', '2026-03-08 13:49:00', '2026-03-08 14:08:00'),
            ('93687f5ecbf47444a47d2f218d9418e013bde6c6d7acc5e02157f8fd6bfc9cad', (select id from users where email = 'lucas.berg@syncmed.no'), 'Omar Hassan', 54, 'male', 'sent', '2026-03-08 16:03:00', null, '2026-03-08 16:24:00');",
        )
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("insert patients failed: {e}")))?;

        sql_query(
            "insert into case_medications (
                patient_id, med_name, dose, frequency, start_date, end_date, notes
            )
            select
                p.id, seed.med_name, seed.dose, seed.frequency, seed.start_date, seed.end_date, seed.notes
            from patient p
            join (
                values
                    ('Alex Wong', 'Paracetamol', '500mg', 'twice a day', date '2026-02-01', null, 'As needed for fever'),
                    ('Nina Park', 'Ibuprofen', '200mg', 'once a day', date '2026-02-10', date '2026-03-10', 'After meal')
            ) as seed(name_snapshot, med_name, dose, frequency, start_date, end_date, notes)
            on p.name_snapshot = seed.name_snapshot;",
        )
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("insert medications failed: {e}")))?;

        sql_query(
            "insert into case_chat_messages (
                patient_id, sender_type, content_text, created_at
            )
            select
                p.id, seed.sender_type, seed.content_text, now()
            from patient p
            join (
                values
                    ('Alex Wong', 'patient', 'I am taking Paracetamol for a recurring headache.'),
                    ('Alex Wong', 'bot', 'Thank you. Please confirm your daily dosage and duration.'),
                    ('Nina Park', 'patient', 'I started Ibuprofen last month and felt better.')
            ) as seed(name_snapshot, sender_type, content_text)
            on p.name_snapshot = seed.name_snapshot;",
        )
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("insert chats failed: {e}")))?;

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "refresh_database is only available on the server".to_string(),
        ))
    }
}
