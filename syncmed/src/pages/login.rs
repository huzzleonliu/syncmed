use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::A;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <Title text="SyncMed Login"/>
        <main class="min-h-screen bg-[#f8fafc] text-[#1e293b]">
            <SyncMedHeader active="login"/>

            <section class="mx-auto w-full max-w-[700px] px-4 py-10 md:py-12">
                <div class="rounded-lg border border-[#e2e8f0] bg-white p-6 md:p-10 shadow-[0px_4px_6px_-1px_rgba(0,0,0,0.10),0px_2px_4px_-1px_rgba(0,0,0,0.06)]">
                    <LoginFormPanel/>
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
fn SyncMedFooter() -> impl IntoView {
    view! {
        <footer class="border-t border-[#e2e8f0] bg-white py-8">
            <p class="text-center text-sm text-[#64748b]">"© 2026 SyncMed. All rights reserved."</p>
        </footer>
    }
}

#[component]
fn LoginFormPanel() -> impl IntoView {
    view! {
        <div class="w-full">
            <h2 class="text-center text-3xl font-semibold text-[#4a6bff]">"Login"</h2>

            <div class="mt-4 text-center text-sm text-[#64748b]">
                <p>"Please provide your medical history to the doctor in advance."</p>
                <p class="mt-1 text-xs">"This will only take 5-10 min"</p>
            </div>

            <label class="mt-7 block">
                <span class="mb-2 block text-sm font-medium text-[#1e293b]">"Email"</span>
                <input
                    type="email"
                    placeholder="m@example.com"
                    class="h-[46px] w-full rounded-lg border border-[#e2e8f0] bg-white px-4 text-base text-[#1e293b] placeholder:text-[#94a3b8] focus:outline-none focus:ring-2 focus:ring-[#4a6bff]/30"
                />
            </label>

            <div class="mt-5 flex items-center justify-between">
                <span class="text-sm font-medium text-[#1e293b]">"Password"</span>
                <a href="#" class="text-sm text-[#4a6bff] hover:underline">"Forgot password?"</a>
            </div>
            <input
                type="password"
                class="mt-2 h-[46px] w-full rounded-lg border border-[#e2e8f0] bg-white px-4 text-base text-[#1e293b] focus:outline-none focus:ring-2 focus:ring-[#4a6bff]/30"
            />

            <A
                href="/patient-information"
                attr:class="mt-6 inline-flex h-[46px] w-full items-center justify-center rounded-lg border border-[#4a6bff] bg-[#4a6bff] px-6 text-base font-medium text-white hover:bg-[#3f5cf0]"
            >
                "Login"
            </A>
        </div>
    }
}
