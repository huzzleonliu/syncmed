use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

#[component]
pub fn PatientInformationPage() -> impl IntoView {
    view! {
        <Title text="Patient Information - SyncMed"/>
        <main class="min-h-screen bg-[#f8fafc] text-[#1e293b]">
            <SyncMedHeader active="login"/>

            <section class="mx-auto w-full max-w-[700px] px-4 py-8 md:py-10">
                <ProgressSteps/>

                <div class="mt-7 rounded-lg border border-[#e2e8f0] bg-white p-6 md:p-10 shadow-[0px_4px_6px_-1px_rgba(0,0,0,0.10),0px_2px_4px_-1px_rgba(0,0,0,0.06)]">
                    <div class="text-center">
                        <h1 class="text-[28px] font-bold leading-[1.3] text-[#1e293b]">"Patient Information"</h1>
                        <p class="mt-2 text-sm text-[#64748b] md:text-base">
                            "Please provide the patient's basic information to begin the medication reconciliation process."
                        </p>
                    </div>

                    <form class="mt-7 space-y-6">
                        <label class="block">
                            <span class="mb-2 block text-base font-medium text-[#1e293b]">"Full Name"</span>
                            <input
                                type="text"
                                placeholder="Enter the patient's full name"
                                class="h-[46px] w-full rounded-lg border border-[#e2e8f0] bg-white px-4 text-base text-[#1e293b] placeholder:text-[#757575] focus:outline-none focus:ring-2 focus:ring-[#4a6bff]/30"
                            />
                        </label>

                        <div class="grid gap-4 md:grid-cols-2">
                            <label class="block">
                                <span class="mb-2 block text-base font-medium text-[#1e293b]">"Age"</span>
                                <input
                                    type="number"
                                    min="0"
                                    placeholder="Age"
                                    class="h-[46px] w-full rounded-lg border border-[#e2e8f0] bg-white px-4 text-base text-[#1e293b] placeholder:text-[#757575] focus:outline-none focus:ring-2 focus:ring-[#4a6bff]/30"
                                />
                            </label>

                            <label class="block">
                                <span class="mb-2 block text-base font-medium text-[#1e293b]">"Gender"</span>
                                <select
                                    class="h-[46px] w-full rounded-lg border border-[#e2e8f0] bg-[#efefef] px-4 text-base text-[#1e293b] focus:outline-none focus:ring-2 focus:ring-[#4a6bff]/30"
                                >
                                    <option selected=true disabled=true value="">"Select gender"</option>
                                    <option value="female">"Female"</option>
                                    <option value="male">"Male"</option>
                                    <option value="other">"Other"</option>
                                </select>
                            </label>
                        </div>

                        <div class="flex items-center justify-between pt-4">
                            <A
                                href="/login"
                                attr:class="inline-flex h-[46px] items-center justify-center rounded-lg border border-[#4a6bff] px-6 text-base font-medium text-[#4a6bff] hover:bg-[#eef2ff]"
                            >
                                "Back"
                            </A>
                            <A
                                href="/review-information"
                                attr:class="inline-flex h-[46px] items-center justify-center rounded-lg border border-[#4a6bff] bg-[#4a6bff] px-6 text-base font-medium text-white hover:bg-[#3f5cf0]"
                            >
                                "Continue to Review"
                            </A>
                        </div>
                    </form>
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
fn ProgressSteps() -> impl IntoView {
    view! {
        <div class="relative flex items-start justify-between">
            <div class="absolute left-[18%] right-[18%] top-[18px] h-[2px] bg-[#e2e8f0]"></div>

            <StepItem number=1 label="Patient Information" is_active=true/>
            <StepItem number=2 label="Review" is_active=false/>
            <StepItem number=3 label="Medication Chat" is_active=false/>
        </div>
    }
}

#[component]
fn StepItem(number: u8, label: &'static str, is_active: bool) -> impl IntoView {
    let circle_class = if is_active {
        "bg-[#4a6bff]"
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
