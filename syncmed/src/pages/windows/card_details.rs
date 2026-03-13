use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsCardDetailsPage() -> impl IntoView {
    view! {
        <Title text="Windows Card Details - SyncMed"/>
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
                            <A href="/windows/generate-url" attr:class="block rounded px-3 py-2 text-black/85 hover:bg-black/5">
                                "Generate Patient URL"
                            </A>
                            <A href="/windows/browse-patient-entry" attr:class="block rounded px-3 py-2 text-black/85 hover:bg-black/5">
                                "Browse Patient Entry"
                            </A>
                            <A href="/windows/card-details" attr:class="block rounded bg-black/5 px-3 py-2 text-black">
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
                            <h1 class="text-[28px] font-semibold text-black/85">"Chat Details"</h1>
                        </div>

                        <div class="flex-1 overflow-auto px-6 pb-8 lg:px-8">
                            <div class="space-y-6">
                                <div class="border-b border-black/10 pb-4">
                                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Patient Info"</h2>
                                    <div class="grid grid-cols-1 gap-3 text-sm text-black/85 sm:grid-cols-3">
                                        <p><span class="font-bold">"Name "</span>"Huzz Liu"</p>
                                        <p><span class="font-bold">"Age "</span>"30"</p>
                                        <p><span class="font-bold">"Gender "</span>"male"</p>
                                    </div>
                                </div>

                                <div class="border-b border-black/10 pb-4">
                                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Medication List"</h2>
                                    <div class="overflow-x-auto rounded-lg">
                                        <table class="table table-sm w-full text-[13px] text-black/85">
                                            <thead class="text-black/90">
                                                <tr>
                                                    <th>"Index"</th>
                                                    <th>"Medication Name"</th>
                                                    <th>"Taking Period"</th>
                                                    <th>"Frequency"</th>
                                                    <th>"Comments"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                <MedicationRow index="1" name="Paracetamol" period="2025 Oct. -> 2026 Jan" frequency="30mg per day"/>
                                                <MedicationRow index="2" name="Ibuprofen" period="2025 Nov. -> 2026 Feb" frequency="200mg per day"/>
                                                <MedicationRow index="3" name="Amoxicillin" period="2025 Oct. -> 2026 Apr" frequency="500mg twice a day"/>
                                                <MedicationRow index="4" name="Aspirin" period="2025 Dec. -> 2026 Mar" frequency="75mg per day"/>
                                            </tbody>
                                        </table>
                                    </div>
                                </div>

                                <div>
                                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Chat History"</h2>
                                    <div class="rounded border border-black/10 bg-white/70 p-4 text-[13px] text-black/80">
                                        <div class="grid grid-cols-[90px,1fr] gap-4">
                                            <p class="font-medium">"Chat Bot"</p>
                                            <p>
                                                "Thank you for sharing that, Benjamin. Just to clarify, when you say \"Paracet,\" are you referring to Paracetamol (also known as acetaminophen)? Could you tell me the dose you usually take and how often you use it?"
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                <div class="flex items-center justify-between pt-1">
                                    <A href="/windows/browse-patient-entry" attr:class="btn btn-ghost btn-sm text-black/70">"Back"</A>
                                    <A href="/" attr:class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]">
                                        "Finish"
                                    </A>
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </main>
    }
}

#[component]
fn MedicationRow(
    index: &'static str,
    name: &'static str,
    period: &'static str,
    frequency: &'static str,
) -> impl IntoView {
    view! {
        <tr class="border-black/5">
            <td>{index}</td>
            <td>{name}</td>
            <td>{period}</td>
            <td>{frequency}</td>
            <td>
                <button type="button" class="btn btn-ghost btn-xs h-6 min-h-6 px-2 text-base leading-none">"💬"</button>
            </td>
        </tr>
    }
}
