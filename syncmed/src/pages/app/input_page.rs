use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const LOGO_GROUP_URL: &str =
    "https://www.figma.com/api/mcp/asset/7b60b857-7bc2-4087-acab-4d5c37a8a322";
const AVATAR_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/73c194b0-97fd-42e5-a422-370d22b926ef";

#[component]
pub fn AppInputPage() -> impl IntoView {
    view! {
        <Title text="App Input Page - SyncMed"/>
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
                <div class="grid gap-6 lg:grid-cols-[1fr_500px]">
                    <CreateEntryCard/>
                    <PatientListCard/>
                </div>
            </section>

            <footer class="border-t border-custom-border bg-custom-card px-6 py-8 text-center text-sm text-custom-muted-foreground">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

#[component]
fn CreateEntryCard() -> impl IntoView {
    view! {
        <div class="card rounded-xl border border-custom-border bg-custom-background shadow-sm">
            <div class="card-body gap-4 p-5 md:p-6">
                <h2 class="text-lg font-bold text-custom-foreground">"Create Entry"</h2>

                <label class="form-control">
                    <span class="label-text mb-2 text-base font-medium text-custom-foreground">"Patient Name"</span>
                    <input
                        type="text"
                        placeholder="Patient Name"
                        class="input input-bordered h-9 w-full border-custom-ring bg-custom-background text-custom-accent-foreground focus:outline-none"
                    />
                </label>

                <div class="grid gap-3 sm:grid-cols-2">
                    <label class="form-control">
                        <span class="label-text mb-2 text-base font-medium text-custom-foreground">"Age"</span>
                        <input
                            type="number"
                            min="0"
                            placeholder="Age"
                            class="input input-bordered h-9 w-full border-custom-ring bg-custom-background text-custom-accent-foreground focus:outline-none"
                        />
                    </label>

                    <label class="form-control">
                        <span class="label-text mb-2 text-base font-medium text-custom-foreground">"Gender"</span>
                        <select class="select select-bordered h-9 min-h-9 w-full border-custom-ring bg-custom-background text-custom-accent-foreground focus:outline-none">
                            <option selected=true disabled=true value="">"Gender"</option>
                            <option value="female">"Female"</option>
                            <option value="male">"Male"</option>
                            <option value="other">"Other"</option>
                        </select>
                    </label>
                </div>

                <div class="mt-2 flex justify-end">
                    <A href="/app/confirm-identity" attr:class="btn btn-primary h-9 min-h-9 px-6">"Create"</A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PatientListCard() -> impl IntoView {
    view! {
        <div class="card rounded-xl border border-custom-border bg-custom-background shadow-sm">
            <div class="border-b border-custom-border px-5 py-3 md:px-6">
                <h3 class="text-lg font-bold text-custom-foreground">"patient list"</h3>
            </div>

            <div class="overflow-x-auto">
                <table class="table table-zebra">
                    <thead>
                        <tr class="text-base font-semibold text-custom-foreground">
                            <th>"Name"</th>
                            <th>"Age"</th>
                            <th>"Gender"</th>
                            <th>"entry creator"</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        <PatientRow name="Huzz Liu" age=30 gender="male" creator="Hilde (nurse)"/>
                        <PatientRow name="Hendirk Helsen" age=20 gender="male" creator="Gao (Doctor)"/>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn PatientRow(
    name: &'static str,
    age: u8,
    gender: &'static str,
    creator: &'static str,
) -> impl IntoView {
    view! {
        <tr class="text-base text-custom-foreground">
            <td>{name}</td>
            <td>{age}</td>
            <td>{gender}</td>
            <td>{creator}</td>
            <td class="text-right">
                <A href="/app/confirm-identity" attr:class="btn btn-primary h-9 min-h-9 px-4">"Choose"</A>
            </td>
        </tr>
    }
}
