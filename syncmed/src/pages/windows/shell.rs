use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::{A, Outlet};
use leptos_router::hooks::use_location;

#[component]
pub fn WindowsLayout() -> impl IntoView {
    let location = use_location();
    let (is_dragging, set_is_dragging) = signal(false);
    let (drag_offset, set_drag_offset) = signal((0, 0));
    let (position, set_position) = signal((0, 0));
    let (scale, set_scale) = signal(1.0_f64);

    let start_drag = move |ev: MouseEvent| {
        ev.prevent_default();
        let (x, y) = position.get();
        set_drag_offset.set((ev.client_x() - x, ev.client_y() - y));
        set_is_dragging.set(true);
    };

    let on_move = move |ev: MouseEvent| {
        if is_dragging.get() {
            let (ox, oy) = drag_offset.get();
            set_position.set((ev.client_x() - ox, ev.client_y() - oy));
        }
    };

    let end_drag = move |_| set_is_dragging.set(false);

    let nav_class = move |target: &'static str| {
        move || {
            let active = location.pathname.get() == target;
            if active {
                "block rounded bg-black/5 px-3 py-2 text-black"
            } else {
                "block rounded px-3 py-2 text-black/85 hover:bg-black/5"
            }
        }
    };

    view! {
        <Title text="Windows App - SyncMed"/>
        <main
            class="relative min-h-screen overflow-hidden p-4 text-black lg:p-10"
            on:mousemove=on_move
            on:mouseup=end_drag
            on:mouseleave=end_drag
        >
            <img
                src="/windows.png"
                alt="Windows desktop background"
                class="pointer-events-none absolute inset-0 h-full w-full object-cover"
            />
            <div class="pointer-events-none absolute inset-0 bg-[linear-gradient(180deg,rgba(8,12,20,0.2)_0%,rgba(9,11,17,0.35)_100%)]"></div>

            <div class="absolute inset-0 flex items-center justify-center">
                <div
                    class="relative w-[min(94vw,1252px)] rounded-[10px] border border-black/25 bg-white shadow-[0_32px_64px_rgba(0,0,0,0.28),0_2px_21px_rgba(0,0,0,0.22)]"
                    style=move || {
                        let (x, y) = position.get();
                        let s = scale.get();
                        format!(
                            "transform: translate({x}px, {y}px) scale({s}); transform-origin: center; resize: both; overflow: auto;"
                        )
                    }
                >
                    <div
                        class="flex h-12 cursor-move select-none items-center justify-between rounded-t-[10px] border-b border-black/10 px-4"
                        on:mousedown=start_drag
                    >
                        <div class="text-xs text-black/85">"SyncMed"</div>
                        <div class="flex items-center gap-2">
                            <button
                                type="button"
                                class="btn btn-ghost btn-xs h-7 min-h-7 w-7 p-0 text-black/80"
                                on:click=move |_| set_scale.update(|v| *v = (*v - 0.1).max(0.7))
                            >
                                "−"
                            </button>
                            <button
                                type="button"
                                class="btn btn-ghost btn-xs h-7 min-h-7 w-7 p-0 text-black/80"
                                on:click=move |_| set_scale.set(1.0)
                            >
                                "□"
                            </button>
                            <button
                                type="button"
                                class="btn btn-ghost btn-xs h-7 min-h-7 w-7 p-0 text-black/80"
                                on:click=move |_| set_scale.update(|v| *v = (*v + 0.1).min(1.3))
                            >
                                "+"
                            </button>
                        </div>
                    </div>

                    <div class="flex min-h-[680px] flex-col lg:min-h-[760px] lg:flex-row">
                        <aside class="w-full border-b border-black/10 bg-white p-2 lg:w-[280px] lg:border-b-0 lg:border-r lg:border-black/10">
                            <div class="mb-2 px-3 py-2 text-xs text-black/80">"SyncMed"</div>
                            <nav class="space-y-1 text-sm">
                                <A href="/windows/login" attr:class=nav_class("/windows/login")>"Login"</A>
                                <A href="/windows/generate-url" attr:class=nav_class("/windows/generate-url")>
                                    "Generate Patient URL"
                                </A>
                                <A href="/windows/browse-patient-entry" attr:class=nav_class("/windows/browse-patient-entry")>
                                    "Browse Patient Entry"
                                </A>
                                <A href="/windows/card-details" attr:class=nav_class("/windows/card-details")>
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
                            <Outlet/>
                        </section>
                    </div>
                </div>
            </div>
        </main>
    }
}
