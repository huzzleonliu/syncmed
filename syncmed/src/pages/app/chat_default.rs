use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const LOGO_GROUP_URL: &str =
    "https://www.figma.com/api/mcp/asset/a5f3a6b1-a02e-4798-b13a-32da13edf8de";
const AVATAR_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/ab19cb29-0b34-41d7-94bc-003ced3c67f8";
const MEDICINE_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/9a57ef9e-5a9e-45f5-878d-3cd0195c7a06";

#[component]
pub fn AppChatDefaultPage() -> impl IntoView {
    view! {
        <Title text="App Chat Default - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
            <header class="border-b border-custom-border bg-custom-card">
                <div class="mx-auto flex h-[85px] w-full max-w-[1280px] items-center justify-between px-4 md:px-6">
                    <div class="flex items-center gap-3">
                        <img src=LOGO_GROUP_URL alt="SyncMed Logo" class="h-8 w-8 object-contain"/>
                        <span class="text-xl font-bold text-custom-primary">"SyncMed"</span>
                    </div>
                    <div class="hidden items-center gap-3 sm:flex">
                        <div class="avatar">
                            <div class="w-8 rounded-full">
                                <img src=AVATAR_IMAGE_URL alt="Nurse avatar"/>
                            </div>
                        </div>
                        <div class="text-sm leading-5 text-custom-accent-foreground">
                            <p class="font-bold">"Nurse"</p>
                            <p class="font-light">"Hilde.C@gmail.com"</p>
                        </div>
                    </div>
                </div>
            </header>

            <section class="mx-auto w-full max-w-[1280px] px-4 py-6 md:px-6 md:py-8">
                <div class="grid gap-4 lg:grid-cols-[2fr_1fr]">
                    <ChatPanel/>
                    <MedicationPanel/>
                </div>
            </section>

            <footer class="border-t border-custom-border bg-custom-card px-6 py-8 text-center text-sm text-custom-muted-foreground">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

#[component]
fn ChatPanel() -> impl IntoView {
    view! {
        <div class="card border border-custom-border bg-custom-background shadow-sm">
            <div class="flex items-center justify-between border-b border-custom-border p-4">
                <h2 class="text-xl font-bold text-custom-foreground">"Medication Reconciliation"</h2>
                <div class="flex items-center gap-2">
                    <A href="/app/chat-accessibility" attr:class="btn btn-primary btn-sm">"Switch to accessibility page"</A>
                    <button type="button" class="btn btn-primary btn-square btn-sm">"↗"</button>
                </div>
            </div>

            <div class="space-y-4 p-4">
                <ChatBubble
                    text="Hello! I'm here to help you create a complete list of medications Huzz. Let's start by discussing any prescription medications you're currently taking. What medications do you take regularly?"
                    time="10:19"
                    from_user=false
                />
                <ChatBubble text="fdfdf" time="10:19" from_user=true/>
                <ChatBubble
                    text="Hello Huzz, I'm here to help you review your medications to make sure everything is accurate and up-to-date. To start, can you tell me the names of any medications or supplements you are currently taking? If you're not sure, do you remember what the medication is for, or what it looks like?"
                    time="10:19"
                    from_user=false
                />
            </div>

            <div class="mt-auto flex items-center gap-3 border-t border-custom-border p-4">
                <textarea
                    class="textarea textarea-bordered h-20 flex-1 resize-none border-custom-input bg-custom-background"
                    placeholder="Type your message here..."
                ></textarea>
                <button type="button" class="btn btn-primary btn-circle">"➤"</button>
                <button type="button" class="btn btn-outline btn-circle border-custom-input">"◍"</button>
            </div>
        </div>
    }
}

#[component]
fn ChatBubble(text: &'static str, time: &'static str, from_user: bool) -> impl IntoView {
    let wrap_class = if from_user {
        "justify-end"
    } else {
        "justify-start"
    };
    let bubble_class = if from_user {
        "bg-primary text-primary-content"
    } else {
        "bg-base-200 text-custom-foreground"
    };

    view! {
        <div class=move || format!("flex {}", wrap_class)>
            <div class=move || format!("max-w-[80%] rounded-lg p-3 {}", bubble_class)>
                <p class="text-sm leading-6 whitespace-pre-wrap">{text}</p>
                <p class="mt-2 text-right text-xs opacity-70">{time}</p>
            </div>
        </div>
    }
}

#[component]
fn MedicationPanel() -> impl IntoView {
    view! {
        <div class="card border border-custom-border bg-custom-background shadow-sm">
            <div class="border-b border-custom-border p-4">
                <h3 class="text-xl font-semibold text-custom-foreground">"Medication List"</h3>
            </div>

            <div class="p-4">
                <div class="rounded-lg border-2 border-dashed border-custom-input bg-custom-card p-2">
                    <MedicineCard/>
                </div>
            </div>

            <div class="p-4 pt-0">
                <A href="/app/confirm-success-page" attr:class="btn btn-primary w-full">"Confirm and Upload"</A>
            </div>
        </div>
    }
}

#[component]
fn MedicineCard() -> impl IntoView {
    view! {
        <div class="card border border-custom-border bg-custom-card">
            <div class="card-body gap-3 p-4">
                <div class="flex items-start gap-3">
                    <img src=MEDICINE_IMAGE_URL alt="Medicine" class="h-[70px] w-[70px] rounded bg-custom-background object-cover p-2"/>
                    <div class="flex-1">
                        <h4 class="text-lg font-bold text-custom-foreground">"medicine name"</h4>
                        <p class="text-base text-custom-primary">"company name"</p>
                        <p class="text-sm text-custom-primary">"Function : Fever reducer, pain reliever"</p>
                    </div>
                    <span class="badge badge-primary">"Generated"</span>
                </div>

                <div class="grid grid-cols-2 gap-3 text-sm">
                    <div>
                        <p class="font-semibold text-custom-foreground">"Taking period"</p>
                        <p class="text-custom-foreground/80">"2025 Oct. -> 2026 Jan. ==> 3 month"</p>
                    </div>
                    <div>
                        <p class="font-semibold text-custom-foreground">"frequency"</p>
                        <p class="text-custom-foreground/80">"1 pill per day"</p>
                    </div>
                </div>

                <div>
                    <p class="mb-1 text-sm font-semibold text-custom-foreground">"Comments"</p>
                    <div class="rounded border-2 border-custom-ring bg-custom-background p-2 text-sm text-custom-foreground">
                        "Prescription from Doctor James"
                    </div>
                </div>
            </div>
        </div>
    }
}
