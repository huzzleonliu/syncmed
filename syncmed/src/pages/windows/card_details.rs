use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsCardDetailsPage() -> impl IntoView {
    view! {
        <Title text="Windows Card Details - SyncMed"/>
        <main class="min-h-screen bg-base-100 p-8 text-base-content">
            <div class="mx-auto max-w-2xl rounded-lg border border-base-300 bg-base-200 p-6">
                <h1 class="text-2xl font-bold">"windows/card-details"</h1>
                <div class="mt-6 flex gap-3">
                    <A href="/windows/browse-patient-entry" attr:class="btn btn-outline">"Back"</A>
                    <A href="/" attr:class="btn btn-primary">"Back to Home"</A>
                </div>
            </div>
        </main>
    }
}
