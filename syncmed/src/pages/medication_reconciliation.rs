use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

#[component]
pub fn MedicationReconciliationPage() -> impl IntoView {
    view! {
        <Title text="Medication Reconciliation - SyncMed"/>
        <main class="min-h-screen bg-[#f8fafc] text-[#1e293b]">
            <SyncMedHeader active="login"/>

            <section class="mx-auto w-full max-w-[1200px] px-6 pb-6 pt-8">
                <div class="grid gap-8 lg:grid-cols-[minmax(0,672px)_minmax(0,448px)]">
                    <section class="flex min-h-[852px] flex-col overflow-hidden rounded-lg border border-[#e2e8f0] bg-white shadow-[0px_4px_6px_-1px_rgba(0,0,0,0.10),0px_2px_4px_-1px_rgba(0,0,0,0.06)]">
                        <header class="flex items-center justify-between border-b border-[#e2e8f0] px-6 pb-[25px] pt-6">
                            <h2 class="text-2xl font-bold text-[#1e293b]">"Medication Reconciliation"</h2>
                            <div class="flex gap-3">
                                <button
                                    type="button"
                                    class="inline-flex h-[46px] items-center justify-center rounded-lg border border-[#4a6bff] px-6 text-base font-medium text-[#4a6bff] hover:bg-[#eef2ff]"
                                >
                                    "Restart"
                                </button>
                                <button
                                    type="button"
                                    class="inline-flex h-[46px] items-center justify-center rounded-lg border border-[#4a6bff] px-6 text-base font-medium text-[#4a6bff] hover:bg-[#eef2ff]"
                                >
                                    "Export"
                                </button>
                            </div>
                        </header>

                        <div class="flex-1 space-y-6 overflow-y-auto p-6">
                            <ChatCard
                                message="Hello! I'm here to help you create a complete list of medications Huzz. Let's start by discussing any prescription medications you're currently taking. What medications do you take regularly?"
                                time="10:19"
                                from_user=false
                            />
                            <ChatCard message="fdfdf" time="10:19" from_user=true/>
                            <ChatCard
                                message="Hello Huzz, I'm here to help you review your medications to make sure everything is accurate and up-to-date. To start, can you tell me the names of any medications or supplements you are currently taking? If you're not sure, do you remember what the medication is for, or what it looks like?"
                                time="10:19"
                                from_user=false
                            />
                        </div>

                        <footer class="flex items-start gap-3 border-t border-[#e2e8f0] px-4 pb-4 pt-[17px]">
                            <textarea
                                class="h-[66px] flex-1 resize-none rounded-lg border border-[#e2e8f0] bg-white p-[13px] text-base placeholder:text-[#757575] focus:outline-none focus:ring-2 focus:ring-[#4a6bff]/30"
                                placeholder="Type your message here..."
                            ></textarea>
                            <button
                                type="button"
                                class="inline-flex size-11 items-center justify-center rounded-full bg-[#4a6bff] text-white"
                            >
                                "➤"
                            </button>
                            <button
                                type="button"
                                class="inline-flex size-11 items-center justify-center rounded-full border-2 border-[#4a6bff] bg-white text-[#4a6bff]"
                            >
                                "🎤"
                            </button>
                        </footer>
                    </section>

                    <section class="flex min-h-[852px] flex-col overflow-hidden rounded-lg border border-[#e2e8f0] bg-white shadow-[0px_4px_6px_-1px_rgba(0,0,0,0.10),0px_2px_4px_-1px_rgba(0,0,0,0.06)]">
                        <header class="flex items-center justify-between border-b border-[#e2e8f0] px-6 pb-[25px] pt-[23px]">
                            <h3 class="text-2xl font-semibold text-[#1e293b]">"Medication List"</h3>
                            <div class="flex flex-col items-end gap-3">
                                <span class="text-sm text-[#64748b]">"0 complete medications"</span>
                                <button
                                    type="button"
                                    class="inline-flex h-[44px] items-center justify-center rounded-lg border border-[#4a6bff] bg-[#4a6bff] px-6 text-base font-medium text-white opacity-60"
                                >
                                    "Complete"
                                </button>
                            </div>
                        </header>

                        <div class="flex-1 p-6">
                            <div class="flex h-full flex-col items-center justify-center rounded-lg border-2 border-dashed border-[#e2e8f0] bg-[#f8fafc] px-8 text-center">
                                <div class="text-5xl text-[#cbd5e1]">"□"</div>
                                <p class="mt-6 text-[20px] font-semibold text-[#334155]">"No complete medications yet"</p>
                                <p class="mt-2 text-sm text-[#64748b]">
                                    "Complete medications with all details will appear here"
                                </p>
                            </div>
                        </div>
                    </section>
                </div>
            </section>

            <SyncMedFooter/>
        </main>
    }
}

#[component]
fn ChatCard(message: &'static str, time: &'static str, from_user: bool) -> impl IntoView {
    let wrapper_class = if from_user {
        "items-end"
    } else {
        "items-start"
    };
    let bubble_class = if from_user {
        "bg-[#4a6bff] text-white border-[#4a6bff]"
    } else {
        "bg-[rgba(226,232,240,0.60)] text-[#334155] border-[#e2e8f0]"
    };
    let time_class = if from_user {
        "text-white/70"
    } else {
        "text-[#334155]/70"
    };

    view! {
        <div class=move || format!("flex w-full flex-col {}", wrapper_class)>
            <div class=move || format!("max-w-[504px] rounded-lg border px-[17px] pb-[17px] pt-4 {}", bubble_class)>
                <p class="whitespace-pre-wrap text-base leading-[25.6px]">{message}</p>
                <div class="mt-2 text-right text-xs leading-[19.2px]">
                    <span class=time_class>{time}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SyncMedHeader(active: &'static str) -> impl IntoView {
    let nav_item = move |href: &'static str, label: &'static str, key: &'static str| {
        let active_class = if active == key {
            "text-[#4a6bff]"
        } else {
            "text-[#334155] hover:text-[#4a6bff]"
        };

        view! {
            <A href=href attr:class=move || format!("text-sm font-medium transition-colors {}", active_class)>
                {label}
            </A>
        }
    };

    view! {
        <header class="border-b border-[#e2e8f0] bg-white">
            <div class="mx-auto flex h-[82px] w-full max-w-[1200px] items-center justify-between px-6 md:px-8">
                <div class="flex items-center gap-3">
                    <span class="inline-flex size-6 items-center justify-center rounded-full bg-[#4a6bff]/10 text-[#4a6bff]">"💊"</span>
                    <span class="text-2xl font-bold text-[#4a6bff]">"SyncMed"</span>
                </div>

                <nav class="flex items-center gap-8">
                    {nav_item("/", "Home", "home")}
                    {nav_item("#", "Dashboard", "dashboard")}
                    {nav_item("/login", "Login", "login")}
                </nav>
            </div>
        </header>
    }
}

#[component]
fn SyncMedFooter() -> impl IntoView {
    view! {
        <footer class="border-t border-[#e2e8f0] bg-white py-8">
            <p class="text-center text-sm text-[#64748b]">"© 2026 SyncMed. All rights reserved."</p>
        </footer>
    }
}
