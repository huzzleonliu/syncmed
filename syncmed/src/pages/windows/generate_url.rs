use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsGenerateUrlPage() -> impl IntoView {
    view! {
        <Title text="Windows Generate URL - SyncMed"/>
        <main class="relative min-h-screen overflow-hidden p-4 text-black lg:p-10">
            <img
                src="/windows.png"
                alt="Windows desktop background"
                class="pointer-events-none absolute inset-0 h-full w-full object-cover"
            />
            <div class="pointer-events-none absolute inset-0 bg-[linear-gradient(180deg,rgba(8,12,20,0.25)_0%,rgba(9,11,17,0.35)_100%)]"></div>

            <div class="relative mx-auto max-w-[1252px] rounded-[10px] border border-black/25 bg-white shadow-[0_32px_64px_rgba(0,0,0,0.28),0_2px_21px_rgba(0,0,0,0.22)]">
                <div class="flex h-12 items-center justify-between rounded-t-[10px] border-b border-black/10 px-4">
                    <div class="text-xs text-black/85">"SyncMed"</div>
                    <div class="flex items-center gap-3">
                        <span class="inline-flex h-5 w-5 items-center justify-center rounded-full bg-black/10 text-[10px] font-medium text-black/70">
                            "JD"
                        </span>
                    </div>
                </div>

                <div class="flex min-h-[680px] flex-col lg:min-h-[760px] lg:flex-row">
                    <aside class="w-full border-b border-black/10 bg-white p-2 lg:w-[280px] lg:border-b-0 lg:border-r lg:border-black/10">
                        <div class="mb-2 px-3 py-2 text-xs text-black/80">"SyncMed"</div>
                        <nav class="space-y-1 text-sm">
                            <A href="/windows/login" attr:class="block rounded px-3 py-2 text-black/85 hover:bg-black/5">"Login"</A>
                            <A href="/windows/generate-url" attr:class="block rounded bg-black/5 px-3 py-2 text-black">
                                "Generate Patient URL"
                            </A>
                            <A href="/windows/browse-patient-entry" attr:class="block rounded px-3 py-2 text-black/85 hover:bg-black/5">
                                "Browse Patient Entries"
                            </A>
                            <A href="/windows/card-details" attr:class="block rounded px-3 py-2 text-black/85 hover:bg-black/5">
                                "Patient Chat Details"
                            </A>
                        </nav>
                        <div class="mt-6 border-t border-black/10 pt-2 lg:mt-auto lg:pt-4">
                            <button type="button" class="btn btn-ghost btn-sm justify-start px-3 font-normal text-black/80">
                                "Settings"
                            </button>
                        </div>
                    </aside>

                    <section class="flex flex-1 flex-col bg-[#f5f5f5]">
                        <div class="px-6 py-6 lg:px-8">
                            <h1 class="text-[28px] font-semibold text-black/85">"Generate Patient URL"</h1>
                        </div>

                        <div class="flex-1 overflow-auto px-6 pb-10 lg:px-8">
                            <div class="mx-auto w-full max-w-[620px] space-y-10">
                                <div>
                                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Create Entry"</h2>
                                    <div class="space-y-3">
                                        <div>
                                            <label class="mb-1 block text-sm text-black/80">"Name"</label>
                                            <input type="text" placeholder="Text" class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"/>
                                        </div>
                                        <div>
                                            <label class="mb-1 block text-sm text-black/80">"Age"</label>
                                            <input type="text" placeholder="Text" class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"/>
                                        </div>
                                        <div>
                                            <label class="mb-1 block text-sm text-black/80">"Gender"</label>
                                            <input type="text" placeholder="Text" class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"/>
                                        </div>
                                    </div>
                                    <div class="mt-4 flex justify-center">
                                        <button type="button" class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]">
                                            "generate URL"
                                        </button>
                                    </div>
                                </div>

                                <div>
                                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Send Link"</h2>
                                    <div class="space-y-3">
                                        <div>
                                            <textarea
                                                placeholder="Text"
                                                class="textarea h-28 w-full rounded border-black/10 bg-white/80 text-black"
                                            ></textarea>
                                        </div>
                                        <div class="flex justify-center">
                                            <button type="button" class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]">
                                                "copy URL"
                                            </button>
                                        </div>
                                        <div>
                                            <label class="mb-1 block text-sm text-black/80">"e-mail address"</label>
                                            <input type="email" placeholder="Text" class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"/>
                                        </div>
                                        <div class="flex items-center justify-between pt-1">
                                            <A href="/windows/browse-patient-entry" attr:class="btn btn-xs min-w-40 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]">
                                                "send URL though email"
                                            </A>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </main>
    }
}
