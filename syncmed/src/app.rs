use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use leptos_router::components::A;


use crate::pages::math_helper::MathHelper;

use crate::pages::math_helper::MathHelperHistory;


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
                <Route path=StaticSegment("math-helper") view=MathHelper/>
                
                <Route
                    path=(StaticSegment("math-helper"), StaticSegment("history"))
                    view=MathHelperHistory
                />
                
            </FlatRoutes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view!{
        <Title text="Home"/>
        <div class="bg-gradient-to-tl from-blue-900 to-blue-100 text-white font-mono flex flex-col min-h-screen">
            <div class="flex flex-col items-start flex-wrap m-auto w-xl">
                <h1 class="text-3xl my-3">Welcome to the website</h1>
                <A href="/math-helper" attr:class="btn btn-primary">Math Helper example</A>
            </div>
        </div>
    }
}
