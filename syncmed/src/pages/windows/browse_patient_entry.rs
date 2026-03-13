use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsBrowsePatientEntryPage() -> impl IntoView {
    view! {
        <Title text="Windows Browse Patient Entry - SyncMed"/>
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
                            <A href="/windows/browse-patient-entry" attr:class="block rounded bg-black/5 px-3 py-2 text-black">
                                "Browse Patient Entry"
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
                            <h1 class="text-[28px] font-semibold text-black/85">"Patient Entries"</h1>
                        </div>

                        <div class="flex-1 overflow-auto px-6 pb-10 lg:px-8">
                            <div class="space-y-5">
                                <div class="flex flex-wrap items-end justify-between gap-4 border-b border-black/10 pb-4">
                                    <div class="card w-[120px] border border-black/10 bg-white shadow-md">
                                        <div class="card-body items-center gap-1 p-3">
                                            <p class="text-[10px] uppercase tracking-wide text-black/45">"Total session"</p>
                                            <p class="text-6xl font-normal leading-none">"15"</p>
                                        </div>
                                    </div>

                                    <div class="flex w-full max-w-[430px] items-end gap-3">
                                        <div class="w-28">
                                            <label class="mb-1 block text-sm text-black/80">"Sort by"</label>
                                            <select class="select select-sm w-full rounded border-black/10 bg-white text-black/80">
                                                <option>"Text"</option>
                                                <option>"Name"</option>
                                                <option>"Status"</option>
                                            </select>
                                        </div>
                                        <div class="flex-1">
                                            <label class="mb-1 block text-sm text-black/80">"Search"</label>
                                            <input
                                                type="text"
                                                placeholder="Search by name"
                                                class="input input-sm w-full rounded border-black/10 bg-white text-black"
                                            />
                                        </div>
                                    </div>
                                </div>

                                <div class="overflow-x-auto rounded-lg">
                                    <table class="table table-sm w-full text-[13px] text-black/85">
                                        <thead class="text-black/90">
                                            <tr>
                                                <th>"Name"</th>
                                                <th>"Age"</th>
                                                <th>"Gender"</th>
                                                <th>"Report Status"</th>
                                                <th>"Sent Time"</th>
                                                <th>"Finish Time"</th>
                                                <th>"Interact"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <PatientRow name="Alex Wong" age="30" gender="female" status="completed" sent="12.34" finish="22.22"/>
                                            <PatientRow name="Lily Zhang" age="25" gender="male" status="pending" sent="18.90" finish="19.50"/>
                                            <PatientRow name="Nina Park" age="32" gender="male" status="completed" sent="11.00" finish="11.05"/>
                                            <PatientRow name="Samir Khan" age="28" gender="female" status="not started" sent="11.11" finish="11.22"/>
                                            <PatientRow name="Alex Wong" age="30" gender="female" status="completed" sent="12.34" finish="22.22"/>
                                            <PatientRow name="Lily Zhang" age="25" gender="male" status="pending" sent="18.90" finish="19.50"/>
                                            <PatientRow name="Nina Park" age="32" gender="male" status="completed" sent="11.00" finish="11.05"/>
                                            <PatientRow name="Samir Khan" age="28" gender="female" status="not started" sent="11.11" finish="11.22"/>
                                        </tbody>
                                    </table>
                                </div>

                                <div class="flex items-center justify-between pt-1">
                                    <A href="/windows/generate-url" attr:class="btn btn-ghost btn-sm text-black/70">"Back"</A>
                                    <A href="/windows/card-details" attr:class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]">
                                        "Next"
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
fn PatientRow(
    name: &'static str,
    age: &'static str,
    gender: &'static str,
    status: &'static str,
    sent: &'static str,
    finish: &'static str,
) -> impl IntoView {
    view! {
        <tr class="border-black/5">
            <td>{name}</td>
            <td>{age}</td>
            <td>{gender}</td>
            <td>{status}</td>
            <td>{sent}</td>
            <td>{finish}</td>
            <td>
                <A href="/windows/card-details" attr:class="btn btn-xs min-h-6 h-6 border-none bg-[#005fb8] px-3 text-white hover:bg-[#0051a0]">
                    "Detail"
                </A>
            </td>
        </tr>
    }
}
