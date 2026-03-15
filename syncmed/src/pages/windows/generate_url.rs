use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use crate::services::patient::create_patient_entry;

#[component]
pub fn WindowsGenerateUrlPage() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (age, set_age) = signal(String::new());
    let (gender, set_gender) = signal(String::new());
    let (generated_url, set_generated_url) = signal(String::new());
    let (error_text, set_error_text) = signal(String::new());
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &create_patient_entry;

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
                            <input
                                type="text"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || name.get()
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label class="mb-1 block text-sm text-black/80">"Age"</label>
                            <input
                                type="text"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || age.get()
                                on:input=move |ev| set_age.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label class="mb-1 block text-sm text-black/80">"Gender"</label>
                            <input
                                type="text"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || gender.get()
                                on:input=move |ev| set_gender.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                    <div class="mt-4 flex justify-center">
                        <button
                            type="button"
                            class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]"
                            on:click=move |_| {
                                set_error_text.set(String::new());
                                let req_name = name.get_untracked();
                                let req_age = age.get_untracked();
                                let req_gender = gender.get_untracked();

                                #[cfg(target_arch = "wasm32")]
                                {
                                    leptos::task::spawn_local(async move {
                                        match create_patient_entry(req_name, req_age, req_gender).await {
                                            Ok(payload) => set_generated_url.set(payload.url),
                                            Err(err) => set_error_text.set(err.to_string()),
                                        }
                                    });
                                }

                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    let _ = (&set_generated_url, req_name, req_age, req_gender);
                                    set_error_text
                                        .set("Generate URL is available after hydration.".to_string());
                                }
                            }
                        >
                            "generate URL"
                        </button>
                    </div>
                    <Show when=move || !error_text.get().is_empty()>
                        <p class="mt-2 text-sm text-red-600">{move || error_text.get()}</p>
                    </Show>
                </div>

                <div>
                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Send Link"</h2>
                    <div class="space-y-3">
                        <div>
                            <textarea
                                placeholder="Generated URL appears here"
                                class="textarea h-28 w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || generated_url.get()
                                readonly
                            ></textarea>
                        </div>
                        <div class="flex justify-center">
                            <button
                                type="button"
                                class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0] disabled:bg-base-300 disabled:text-base-content/60"
                                disabled=move || generated_url.get().is_empty()
                                on:click=move |_| {
                                    let url = generated_url.get_untracked();
                                    if url.is_empty() {
                                        return;
                                    }
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        if let Some(window) = web_sys::window() {
                                            let clipboard = window.navigator().clipboard();
                                            let _ = clipboard.write_text(&url);
                                        }
                                    }
                                }
                            >
                                "copy URL"
                            </button>
                        </div>
                        <div>
                            <label class="mb-1 block text-sm text-black/80">"e-mail address"</label>
                            <input
                                type="email"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                            />
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

