use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const LOGO_GROUP_URL: &str =
    "https://www.figma.com/api/mcp/asset/6f805919-02fb-42fa-bff3-e0bb62fe4a2b";
const AVATAR_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/37f0cd67-8cb9-4fcc-a62e-abf8704a6046";
const HELLO_ICON_URL: &str =
    "https://www.figma.com/api/mcp/asset/00f7917e-d891-440b-960b-14150b9d2c4c";

#[component]
pub fn AppConfirmIdentityPage() -> impl IntoView {
    view! {
        <Title text="App Confirm Identity - SyncMed"/>
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

            <section class="mx-auto w-full max-w-[1280px] px-4 py-8 md:px-6 md:py-10">
                <div class="mx-auto flex max-w-[1108px] flex-col items-center justify-between gap-8 lg:flex-row lg:items-center">
                    <IdentitySummary/>
                    <ConfirmPanel/>
                </div>
            </section>

            <footer class="border-t border-custom-border bg-custom-card px-6 py-8 text-center text-sm text-custom-muted-foreground">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

#[component]
fn IdentitySummary() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-[550px] flex-col items-center gap-6 lg:gap-8">
            <div class="flex items-center gap-4 lg:gap-6">
                <h1 class="text-4xl font-bold text-custom-primary md:text-5xl">"Hi!"</h1>
                <img src=HELLO_ICON_URL alt="Hello icon" class="h-14 w-14 object-contain md:h-[72px] md:w-[72px]"/>
            </div>

            <div class="w-full max-w-[550px] text-center">
                <div class="space-y-1 md:space-y-2">
                    <p class="text-3xl font-bold text-custom-foreground md:text-4xl">"Name"</p>
                    <p class="text-3xl font-light text-custom-primary md:text-4xl">"Huzz Liu"</p>
                </div>

                <div class="mt-6 grid grid-cols-2 gap-6 md:mt-8 md:gap-8">
                    <div class="space-y-1 md:space-y-2">
                        <p class="text-3xl font-bold text-custom-foreground md:text-4xl">"Gender"</p>
                        <p class="text-3xl font-light text-custom-primary md:text-4xl">"male"</p>
                    </div>
                    <div class="space-y-1 md:space-y-2">
                        <p class="text-3xl font-bold text-custom-foreground md:text-4xl">"Age"</p>
                        <p class="text-3xl font-light text-custom-primary md:text-4xl">"30"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ConfirmPanel() -> impl IntoView {
    let (need_accessibility, set_need_accessibility) = signal(false);

    view! {
        <div class="flex w-full max-w-[460px] flex-col gap-5 rounded-xl bg-custom-background/25 p-2 md:p-0">
            <label class="label cursor-pointer justify-start gap-3 md:gap-4">
                <input type="checkbox" checked=true class="checkbox checkbox-primary checkbox-sm md:checkbox-md rounded-[10px]"/>
                <span class="label-text text-lg font-light text-custom-primary md:text-2xl">
                    "I confirm the personal information is correct"
                </span>
            </label>

            <label class="label cursor-pointer justify-start gap-3 md:gap-4">
                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm md:checkbox-md rounded-[10px]"/>
                <span class="label-text text-lg font-light text-custom-primary md:text-2xl">
                    "I agree to the Terms of Service and Privacy Policy."
                </span>
            </label>

            <div class="rounded-[15px] border-2 border-dashed border-info bg-custom-background/40 px-3 py-2 md:px-4 md:py-3">
                <label class="label cursor-pointer justify-start gap-3 md:gap-4">
                    <input
                        type="checkbox"
                        class="checkbox checkbox-primary checkbox-sm md:checkbox-md rounded-[10px]"
                        prop:checked=move || need_accessibility.get()
                        on:change=move |ev| set_need_accessibility.set(event_target_checked(&ev))
                    />
                    <span class="label-text text-lg font-light text-custom-primary md:text-2xl">"I need accessibility layout."</span>
                </label>
            </div>

            <div class="flex justify-center pt-1 md:pt-3">
                <A
                    href=move || {
                        if need_accessibility.get() {
                            "/app/chat-accessibility".to_string()
                        } else {
                            "/app/chat-default".to_string()
                        }
                    }
                    attr:class="btn btn-primary h-12 min-h-12 px-8 text-xl font-medium md:h-[87px] md:min-h-[87px] md:px-12 md:text-4xl"
                >
                    "Confirm"
                </A>
            </div>

            <div class="flex justify-center">
                <A href="/app/input-page" attr:class="btn btn-outline">"Back"</A>
            </div>
        </div>
    }
}
