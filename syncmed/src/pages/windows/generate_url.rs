use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsGenerateUrlPage() -> impl IntoView {
    view! {
        <Title text="Windows Generate URL - SyncMed"/>
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
    }
}
