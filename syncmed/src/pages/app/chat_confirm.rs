use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};

const MEDICINE_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/805648dc-bced-4bde-bf77-9325d2c0ce72";

#[component]
pub fn AppChatConfirmPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });

    view! {
        <Title text="Chat Confirm - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
            <section class="mx-auto w-full max-w-[1024px] px-2 py-2 md:px-4 md:py-4">
                <div class="card border border-custom-ring bg-custom-background shadow-md">
                    <div class="border-b border-custom-border p-4 text-center">
                        <h2 class="text-5xl font-semibold">"Medication List"</h2>
                    </div>

                    <div class="p-6">
                        <div class="rounded-lg border-2 border-dashed border-custom-border bg-custom-card p-4">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-6">
                                <MedicineCard short_comment=false/>
                                <div class="hidden lg:block">
                                    <MedicineCard short_comment=true/>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="pb-6 text-center">
                        <A
                            href=move || {
                                let key = patient_key.get();
                                if key.trim().is_empty() {
                                    "/app/confirm-success-page".to_string()
                                } else {
                                    format!("/app/confirm-success-page?patient-id={key}")
                                }
                            }
                            attr:class="btn btn-primary px-10 text-5xl font-bold"
                        >
                            "Confirm and Upload"
                        </A>
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
fn MedicineCard(short_comment: bool) -> impl IntoView {
    view! {
        <div class="card border border-custom-border bg-custom-card">
            <div class="card-body gap-4 p-4 lg:gap-3 lg:p-5">
                <div class="flex items-start gap-3">
                    <img src=MEDICINE_IMAGE_URL alt="Medicine" class="h-[70px] w-[80px] object-cover lg:h-[55px] lg:w-[70px]"/>
                    <div class="flex-1">
                        <p class="text-6xl font-bold text-custom-foreground lg:text-5xl">"medicine name"</p>
                        <p class="text-4xl text-custom-primary lg:text-3xl">"company name"</p>
                        <p class="text-5xl font-light text-custom-primary lg:text-4xl">"Function : Fever reducer, pain reliever"</p>
                    </div>
                </div>

                <div class="grid grid-cols-2 gap-6 text-4xl lg:gap-3 lg:text-2xl">
                    <div>
                        <p class="font-semibold">"Taking period"</p>
                        <p class="font-normal">"2025 Oct. -> 2026 Jan. => 3 month"</p>
                    </div>
                    <div>
                        <p class="font-semibold">"frequency"</p>
                        <p class="font-normal">"1 pill per day"</p>
                    </div>
                </div>

                <div>
                    <p class="mb-2 text-4xl font-semibold lg:text-2xl">"Comments"</p>
                    <div class=move || {
                        if short_comment {
                            "rounded border-2 border-custom-ring bg-custom-background p-2 text-4xl lg:min-h-[56px] lg:text-2xl"
                        } else {
                            "min-h-[220px] rounded border-2 border-custom-ring bg-custom-background p-2 text-4xl lg:min-h-[220px] lg:text-2xl"
                        }
                    }>
                        "Prescription from Doctor James"
                    </div>
                </div>
            </div>
        </div>
    }
}
