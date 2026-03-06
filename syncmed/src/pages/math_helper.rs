use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <Title text="SyncMed Login"/>
        <main class="min-h-screen bg-[#4a4a4a] text-custom-foreground px-4 py-6 md:px-6 md:py-10">
            <div class="mx-auto w-full max-w-[1200px] rounded-lg bg-custom-card overflow-hidden shadow-sm">
                <header class="h-14 md:h-16 border-b border-custom-border bg-custom-card flex items-center justify-between px-6 md:px-10">
                    <div class="flex items-center gap-2 text-custom-primary font-semibold text-xl">
                        <span class="text-sm">"💊"</span>
                        <span>"SyncMed"</span>
                    </div>
                    <nav class="hidden xl:flex items-center gap-10 text-sm text-custom-foreground">
                        <a href="#" class="hover:text-custom-primary">"Home"</a>
                        <a href="#" class="hover:text-custom-primary">"Dashboard"</a>
                        <a href="#" class="hover:text-custom-primary font-medium">"Login"</a>
                    </nav>
                </header>

                <section class="bg-custom-subtle-background min-h-[660px] md:min-h-[720px] px-4 py-10 md:px-8 md:py-14">
                    <div class="mx-auto w-full max-w-[760px] rounded-lg border border-custom-border bg-custom-card p-4 md:p-8">
                        <div class="grid gap-8 lg:grid-cols-[1fr_220px] lg:items-center">
                            <LoginFormPanel/>
                            <div class="hidden lg:flex items-center justify-center">
                                <div class="size-16 rounded-full border border-custom-border bg-custom-subtle-background flex items-center justify-center text-custom-foreground/40">
                                    "⊗"
                                </div>
                            </div>
                        </div>
                    </div>

                    <p class="text-center text-xs text-custom-foreground/70 mt-8">
                        "By clicking continue, you agree to our "
                        <a href="#" class="underline">"Terms of Service"</a>
                        " and "
                        <a href="#" class="underline">"Privacy Policy"</a>
                        "."
                    </p>
                </section>

                <footer class="h-14 md:h-16 border-t border-custom-border bg-custom-card flex items-center justify-center text-xs text-custom-foreground/70">
                    "© 2026 SyncMed. All rights reserved."
                </footer>
            </div>
        </main>
    }
}

#[component]
fn LoginFormPanel() -> impl IntoView {
    view! {
        <div class="w-full">
            <h2 class="text-3xl font-semibold text-custom-primary text-center lg:text-left">"Login"</h2>

            <div class="mt-4 text-sm text-custom-foreground/80 text-center lg:text-left">
                <p>"Please provide your medical history to the doctor in advance."</p>
                <p class="mt-1 text-xs text-custom-foreground/70">"This will only take 5-10 min"</p>
            </div>

            <label class="form-control w-full mt-7">
                <span class="label-text text-custom-foreground font-medium mb-1">"Email"</span>
                <input
                    type="email"
                    placeholder="m@example.com"
                    class="input input-bordered w-full bg-custom-background"
                />
            </label>

            <div class="mt-5 flex items-center justify-between">
                <span class="text-sm font-medium text-custom-foreground">"Password"</span>
                <a href="#" class="text-sm text-custom-primary hover:underline">"Forgot password?"</a>
            </div>
            <input
                type="password"
                placeholder=""
                class="input input-bordered w-full mt-1 bg-custom-background"
            />

            <button class="btn btn-primary w-full mt-6">"Login"</button>

            <div class="mt-6 text-xs text-center text-custom-foreground/70">"OR CONTINUE WITH"</div>

            <div class="mt-3 grid grid-cols-3 gap-3">
                <button class="btn btn-outline btn-sm bg-custom-background border-custom-border">""</button>
                <button class="btn btn-outline btn-sm bg-custom-background border-custom-border">"G"</button>
                <button class="btn btn-outline btn-sm bg-custom-background border-custom-border">"∞"</button>
            </div>

            <p class="mt-5 text-sm text-center text-custom-foreground">
                "Don't have an account? "
                <a href="#" class="text-custom-primary hover:underline">"Sign up"</a>
                </p>
        </div>
    }
}
