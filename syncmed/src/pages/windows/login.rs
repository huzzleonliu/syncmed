use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsLoginPage() -> impl IntoView {
    view! {
        <Title text="Windows Login - SyncMed"/>
        <div class="px-6 py-6 lg:px-8">
            <h1 class="text-[28px] font-semibold text-black/85">"Login"</h1>
        </div>
        <div class="flex flex-1 items-center justify-center px-6 pb-16">
            <div class="w-full max-w-[360px]">
                <label class="mb-1 block text-sm text-black/85">"Login"</label>
                <input type="text" placeholder="Text" class="input input-sm mb-3 w-full rounded border-black/10 bg-white/80 text-black"/>

                <label class="mb-1 block text-sm text-black/85">"Password"</label>
                <input type="password" placeholder="Text" class="input input-sm mb-8 w-full rounded border-black/10 bg-white/80 text-black"/>

                <div class="flex justify-center">
                    <A href="/windows/generate-url" attr:class="btn btn-sm min-w-24 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]">
                        "Login"
                    </A>
                </div>
            </div>
        </div>
    }
}
