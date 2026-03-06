use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;
use leptos_router::{
    StaticSegment,
    components::{FlatRoutes, Route, Router},
};

use crate::pages::math_helper::LoginPage;

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
                <Route path=StaticSegment("login") view=LoginPage/>
            </FlatRoutes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <div class="bg-base-100 text-base-content flex flex-col min-h-screen">
            <div class="flex flex-col items-start flex-wrap m-auto w-xl">
                <h1 class="text-3xl my-3">"Welcome to SyncMed"</h1>
                <A href="/login" attr:class="btn btn-primary">"Open login layouts"</A>
            </div>
        </div>
    }
}
