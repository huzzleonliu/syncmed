use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsWidgetPage() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <Title text="Windows Widget - SyncMed"/>
        <main class="relative min-h-screen overflow-hidden bg-[#dcdcdc] text-black">
            <img
                src="/windows.png"
                alt="Windows desktop background"
                class="pointer-events-none absolute inset-0 h-full w-full object-cover opacity-65"
            />

            <img
                src="/medical-system.png"
                alt="Medical system overlay"
                class="pointer-events-none absolute bottom-0 left-1/2 w-[100vw] max-w-none -translate-x-1/2 border-t border-black/10 object-cover shadow-md"
            />

            <section class="relative mx-auto min-h-screen w-full max-w-[1280px] px-6 pb-20 pt-8">
                <div class="absolute right-10 top-8 md:right-16">
                    <button
                        type="button"
                        class="group relative rounded-2xl border border-custom-ring bg-custom-background/85 p-2 shadow-lg backdrop-blur transition-transform hover:-translate-y-0.5"
                        on:click=move |_| set_is_open.update(|v| *v = !*v)
                    >
                        <SyncMedTile badge=Some("2")/>
                    </button>
                </div>

                {move || if is_open.get() {
                    view! {
                        <div class="absolute right-10 top-24 md:right-16 md:top-24">
                            <div class="origin-top-right rounded-[22px] border border-custom-ring bg-custom-subtle-background p-4 shadow-[0_16px_35px_rgba(0,0,0,0.16)] animate-[fadeIn_160ms_ease-out]">
                                <div class="mb-3 flex items-center gap-3">
                                    <SyncMedTile badge=Some("2")/>
                                    <div>
                                        <p class="text-sm font-semibold text-custom-foreground">"SyncMed Widget"</p>
                                        <p class="text-xs text-custom-primary">"Select entry to continue"</p>
                                    </div>
                                </div>
                                <PatientListCard/>
                            </div>
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </section>
        </main>
    }
}

#[component]
fn SyncMedTile(badge: Option<&'static str>) -> impl IntoView {
    view! {
        <div class="relative flex h-12 w-12 items-center justify-center rounded-2xl border border-custom-ring/70 bg-custom-background/90 backdrop-blur">
            <div class="h-8 w-8 rounded-lg bg-custom-primary/85"></div>
            {move || {
                badge.map(|value| {
                    view! {
                        <span class="absolute -right-1 -top-1 inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-red-500 px-1 text-[10px] font-semibold text-white">
                            {value}
                        </span>
                    }
                })
            }}
        </div>
    }
}

#[component]
fn PatientListCard() -> impl IntoView {
    view! {
        <div class="w-[230px] rounded-2xl border-2 border-custom-ring bg-custom-subtle-background p-3">
            <div class="mb-3 grid grid-cols-2 gap-2 text-center">
                <div class="rounded-xl border border-custom-ring bg-custom-background py-2">
                    <p class="text-xs text-custom-card-foreground">"New Entry"</p>
                    <p class="text-lg font-bold text-custom-foreground">"3"</p>
                </div>
                <div class="rounded-xl border border-custom-ring bg-custom-background py-2">
                    <p class="text-xs text-custom-card-foreground">"Total Entry"</p>
                    <p class="text-lg font-bold text-custom-foreground">"21"</p>
                </div>
            </div>
            <div class="space-y-1.5">
                <A
                    href="/windows/login"
                    attr:class="block rounded border border-custom-primary/40 bg-custom-secondary px-3 py-1 text-sm font-semibold text-custom-foreground hover:bg-custom-secondary"
                >
                    "Huzz Liu   30   male"
                </A>
                <A
                    href="/windows/login"
                    attr:class="block rounded border border-custom-primary/40 bg-custom-secondary px-3 py-1 text-sm font-semibold text-custom-foreground hover:bg-custom-secondary"
                >
                    "Huzz Liu   30   male"
                </A>
                <A
                    href="/windows/login"
                    attr:class="block rounded border border-custom-primary/40 bg-custom-secondary px-3 py-1 text-sm font-semibold text-custom-foreground hover:bg-custom-secondary"
                >
                    "Huzz Liu   30   male"
                </A>
                <div class="rounded border border-custom-ring/40 bg-custom-background px-3 py-1 text-sm text-custom-card-foreground">
                    "Huzz Liu   30   male"
                </div>
            </div>
        </div>
    }
}
