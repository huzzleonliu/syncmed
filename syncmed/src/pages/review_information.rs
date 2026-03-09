use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

#[component]
pub fn ReviewInformationPage() -> impl IntoView {
    view! {
        <Title text="Review Information - SyncMed"/>
        <main class="min-h-screen bg-[#f8fafc] text-[#1e293b]">
            <SyncMedHeader active="login"/>

            <section class="mx-auto w-full max-w-[700px] px-4 py-8 md:py-10">
                <ReviewProgressSteps/>

                <div class="mt-7 rounded-lg border border-[#e2e8f0] bg-white p-6 md:p-10 shadow-[0px_4px_6px_-1px_rgba(0,0,0,0.10),0px_2px_4px_-1px_rgba(0,0,0,0.06)]">
                    <div class="text-center">
                        <h1 class="text-[28px] font-bold leading-[1.3] text-[#1e293b]">"Review Information"</h1>
                        <p class="mt-2 text-sm text-[#64748b] md:text-base">
                            "Please review the patient information before proceeding to the medication reconciliation."
                        </p>
                    </div>

                    <div class="mt-7 rounded-lg bg-[#4a6bff0d] p-6">
                        <div class="flex h-[50px] items-center border-b border-[#e2e8f0]">
                            <span class="w-[120px] text-base font-semibold text-[#1e293b]">"Full Name:"</span>
                            <span class="text-base text-[#334155]">"Huzz"</span>
                        </div>
                        <div class="flex h-[50px] items-center border-b border-[#e2e8f0]">
                            <span class="w-[120px] text-base font-semibold text-[#1e293b]">"Age:"</span>
                            <span class="text-base text-[#334155]">"30"</span>
                        </div>
                        <div class="flex h-[50px] items-center">
                            <span class="w-[120px] text-base font-semibold text-[#1e293b]">"Gender:"</span>
                            <span class="text-base text-[#334155]">"male"</span>
                        </div>
                    </div>

                    <div class="mt-7 flex items-center justify-between">
                        <A
                            href="/patient-information"
                            attr:class="inline-flex h-[46px] items-center justify-center rounded-lg border border-[#4a6bff] px-6 text-base font-medium text-[#4a6bff] hover:bg-[#eef2ff]"
                        >
                            "Edit Information"
                        </A>
                        <A
                            href="/medication-reconciliation"
                            attr:class="inline-flex h-[46px] items-center justify-center rounded-lg border border-[#4a6bff] bg-[#4a6bff] px-6 text-base font-medium text-white hover:bg-[#3f5cf0]"
                        >
                            "Start Medication Reconciliation"
                        </A>
                    </div>
                </div>
            </section>

            <SyncMedFooter/>
        </main>
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
fn ReviewProgressSteps() -> impl IntoView {
    view! {
        <div class="relative flex items-start justify-between">
            <div class="absolute left-[18%] right-[18%] top-[18px] h-[2px] bg-[#e2e8f0]"></div>
            <div class="absolute left-[18%] right-[52%] top-[18px] h-[2px] bg-[#4a6bff]"></div>

            <StepItem number=1 label="Patient Information" is_active=false/>
            <StepItem number=2 label="Review" is_active=true/>
            <StepItem number=3 label="Medication Chat" is_active=false/>
        </div>
    }
}

#[component]
fn StepItem(number: u8, label: &'static str, is_active: bool) -> impl IntoView {
    let circle_class = if is_active {
        "bg-[#4a6bff]"
    } else if number == 1 {
        "bg-[#38b2ac]"
    } else {
        "bg-[#64748b]"
    };

    let text_class = if is_active {
        "text-[#4a6bff] font-semibold"
    } else {
        "text-[#64748b] font-medium"
    };

    view! {
        <div class="relative z-10 flex flex-col items-center">
            <div class=move || format!("flex size-9 items-center justify-center rounded-full text-base font-semibold text-white {}", circle_class)>
                {number}
            </div>
            <span class=move || format!("mt-2 text-sm {}", text_class)>{label}</span>
        </div>
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
