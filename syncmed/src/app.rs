use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::{
    StaticSegment,
    components::{FlatRoutes, Route, Router},
};

use crate::pages::app::{
    AppChatAccessibilityPage, AppChatConfirmPage, AppChatDefaultPage, AppConfirmIdentityPage,
    AppConfirmSuccessPage, AppInputPage, AppLoginPage,
};
use crate::pages::windows::{
    WindowsBrowsePatientEntryPage, WindowsCardDetailsPage, WindowsGenerateUrlPage,
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

    view! {
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
                <Route path=(StaticSegment("app"), StaticSegment("login")) view=AppLoginPage/>
                <Route path=(StaticSegment("app"), StaticSegment("input-page")) view=AppInputPage/>
                <Route path=(StaticSegment("app"), StaticSegment("confirm-identity")) view=AppConfirmIdentityPage/>
                <Route path=(StaticSegment("app"), StaticSegment("chat-default")) view=AppChatDefaultPage/>
                <Route path=(StaticSegment("app"), StaticSegment("chat-accessibility")) view=AppChatAccessibilityPage/>
                <Route path=(StaticSegment("app"), StaticSegment("chat-confirm")) view=AppChatConfirmPage/>
                <Route path=(StaticSegment("app"), StaticSegment("confirm-success-page")) view=AppConfirmSuccessPage/>
                <Route path=(StaticSegment("windows"), StaticSegment("widget")) view=WindowsWidgetPage/>
                <Route path=(StaticSegment("windows"), StaticSegment("login")) view=WindowsLoginPage/>
                <Route path=(StaticSegment("windows"), StaticSegment("generate-url")) view=WindowsGenerateUrlPage/>
                <Route path=(StaticSegment("windows"), StaticSegment("browse-patient-entry")) view=WindowsBrowsePatientEntryPage/>
                <Route path=(StaticSegment("windows"), StaticSegment("card-details")) view=WindowsCardDetailsPage/>
            </FlatRoutes>
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
                <A href="/windows/widget" attr:class="link">"/windows/widget"</A>
                <A href="/windows/login" attr:class="link">"/windows/login"</A>
                <A href="/windows/generate-url" attr:class="link">"/windows/generate-url"</A>
                <A href="/windows/browse-patient-entry" attr:class="link">"/windows/browse-patient-entry"</A>
                <A href="/windows/card-details" attr:class="link">"/windows/card-details"</A>
            </div>
        </div>
    }
}
