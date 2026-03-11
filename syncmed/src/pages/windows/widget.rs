use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsWidgetPage() -> impl IntoView {
    view! {
        <Title text="Windows Widget - SyncMed"/>
        <main class="min-h-screen bg-base-100 p-8 text-base-content">
            <div class="mx-auto max-w-2xl rounded-lg border border-base-300 bg-base-200 p-6">
                <h1 class="text-2xl font-bold">"windows/widget"</h1>
                <A href="/windows/login" attr:class="btn btn-primary mt-6">"Next: windows/login"</A>
            </div>
        </main>
    }
}
