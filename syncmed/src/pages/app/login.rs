use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const LOGO_GROUP_URL: &str = "/logo.svg";
const PLACEHOLDER_URL: &str =
    "https://www.figma.com/api/mcp/asset/50689abf-6ab5-4055-978f-e47524f963ef";
const APPLE_ICON_URL: &str =
    "https://www.figma.com/api/mcp/asset/3fa3c31f-ac42-49fa-8796-787a1ded5b16";
const GOOGLE_ICON_URL: &str =
    "https://www.figma.com/api/mcp/asset/cf03707d-61bb-4a4d-9641-3368eb2310f6";
const META_ICON_URL: &str =
    "https://www.figma.com/api/mcp/asset/1db8dfeb-a8bb-44eb-a917-43c1656c1981";

#[component]
pub fn AppLoginPage() -> impl IntoView {
    view! {
        <Title text="App Login - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
            <DesktopHeader/>
            <TabletHeader/>

            <section class="mx-auto w-full max-w-[1440px] px-5 pb-10 pt-10 lg:pb-14 lg:pt-16">
                <div class="mx-auto max-w-[894px]">
                    <div class="card overflow-hidden border border-custom-border bg-custom-card shadow-sm">
                        <div class="grid lg:grid-cols-2">
                            <LoginFormArea/>
                            <div class="hidden items-center justify-center bg-custom-muted p-8 lg:flex">
                                <img src=PLACEHOLDER_URL alt="Illustration" class="h-[150px] w-[150px] object-contain"/>
                            </div>
                        </div>
                    </div>

                    <TermsText/>
                </div>
            </section>

            <footer class="hidden border-t border-custom-border bg-custom-card px-8 py-8 text-center text-sm text-custom-muted-foreground md:block">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

#[component]
fn DesktopHeader() -> impl IntoView {
    view! {
        <header class="hidden border-b border-custom-border bg-custom-card lg:block">
            <div class="mx-auto flex h-[85px] w-full max-w-[1200px] items-center justify-between px-6">
                <SyncMedLogo/>
                
            </div>
        </header>
    }
}

#[component]
fn TabletHeader() -> impl IntoView {
    view! {
        <header class="hidden border-b border-custom-border bg-custom-card md:block lg:hidden">
            <div class="mx-auto flex h-[85px] w-full max-w-[1024px] items-center justify-center px-6">
                <SyncMedLogo/>
            </div>
        </header>
    }
}

#[component]
fn SyncMedLogo() -> impl IntoView {
    view! {
        <div class="flex items-center gap-3">
            <img src=LOGO_GROUP_URL alt="SyncMed Logo" class="h-9 w-9 object-contain"/>
            <span class="text-[32px] font-bold leading-none text-custom-primary md:text-2xl">"SyncMed"</span>
        </div>
    }
}

#[component]
fn LoginFormArea() -> impl IntoView {
    view! {
        <div class="p-8">
            <div class="text-center">
                <h1 class="text-2xl font-semibold text-custom-primary">"Login"</h1>
                <p class="mt-2 text-sm text-custom-accent-foreground">
                    "Please provide your medical history to the doctor in advance."
                </p>
                <p class="mt-1 text-right text-xs font-light text-custom-muted-foreground">"This will only take 5-10 min"</p>
            </div>

            <div class="mt-7 space-y-4">
                <label class="block">
                    <span class="mb-2 block text-sm font-medium text-custom-accent-foreground">"Email"</span>
                    <input
                        type="email"
                        value="m@example.com"
                        class="input input-bordered h-9 w-full border-custom-input bg-custom-background text-custom-foreground"
                    />
                </label>

                <label class="block">
                    <div class="mb-2 flex items-center justify-between">
                        <span class="text-sm font-medium text-custom-accent-foreground">"Password"</span>
                        <a href="#" class="link link-hover text-sm text-custom-primary no-underline">"Forgot password?"</a>
                    </div>
                    <input
                        type="password"
                        class="input input-bordered h-9 w-full border-custom-input bg-custom-background text-custom-foreground"
                    />
                </label>
            </div>

            <A
                href="/app/input-page"
                attr:class="btn btn-primary mt-6 h-9 w-full min-h-9"
            >
                "Login"
            </A>

            <div class="divider my-5 text-xs text-custom-muted-foreground">"OR CONTINUE WITH"</div>

            <div class="grid grid-cols-3 gap-4">
                <SocialButton icon_url=APPLE_ICON_URL alt="Apple"/>
                <SocialButton icon_url=GOOGLE_ICON_URL alt="Google"/>
                <SocialButton icon_url=META_ICON_URL alt="Meta"/>
            </div>

            <p class="mt-5 text-center text-sm text-custom-accent-foreground">
                "Don't have an account? "
                <a href="#" class="link link-hover text-custom-primary">"Sign up"</a>
            </p>
        </div>
    }
}

#[component]
fn SocialButton(icon_url: &'static str, alt: &'static str) -> impl IntoView {
    view! {
        <button
            type="button"
            class="btn btn-outline h-9 min-h-9 border-custom-input bg-custom-background"
        >
            <img src=icon_url alt=alt class="h-4 w-4 object-contain"/>
        </button>
    }
}

#[component]
fn TermsText() -> impl IntoView {
    view! {
        <p class="mx-auto mt-6 max-w-[894px] px-4 text-center text-xs text-custom-muted-foreground">
            "By clicking continue, you agree to our "
            <a href="#" class="link link-hover text-custom-muted-foreground">"Terms of Service"</a>
            " and "
            <a href="#" class="link link-hover text-custom-muted-foreground">"Privacy Policy"</a>
            "."
        </p>
    }
}
