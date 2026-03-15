use leptos::prelude::*;
use leptos::ev::MouseEvent;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn WindowsWidgetPage() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (position, set_position) = signal((24_i32, 120_i32));
    let (is_dragging, set_is_dragging) = signal(false);
    let (drag_offset, set_drag_offset) = signal((0_i32, 0_i32));
    let (press_point, set_press_point) = signal((0_i32, 0_i32));
    let (moved_since_press, set_moved_since_press) = signal(false);

    let start_drag = move |ev: MouseEvent| {
        ev.prevent_default();
        let (x, y) = position.get_untracked();
        set_drag_offset.set((ev.client_x() - x, ev.client_y() - y));
        set_press_point.set((ev.client_x(), ev.client_y()));
        set_moved_since_press.set(false);
        set_is_dragging.set(true);
    };

    let on_move = move |ev: MouseEvent| {
        if is_dragging.get() {
            let (ox, oy) = drag_offset.get_untracked();
            set_position.set((ev.client_x() - ox, ev.client_y() - oy));

            let (sx, sy) = press_point.get_untracked();
            if (ev.client_x() - sx).abs() > 3 || (ev.client_y() - sy).abs() > 3 {
                set_moved_since_press.set(true);
            }
        }
    };

    let end_drag = move |_| {
        set_is_dragging.set(false);
    };

    let toggle_popup = move |_| {
        if moved_since_press.get_untracked() {
            set_moved_since_press.set(false);
            return;
        }
        set_is_open.update(|v| *v = !*v);
    };
    let filled_count = Resource::new(|| (), |_| get_filled_patient_count());

    view! {
        <Title text="Windows Widget - SyncMed"/>
        <main
            class="relative min-h-screen overflow-hidden bg-[#dcdcdc] text-black"
            on:mousemove=on_move
            on:mouseup=end_drag
            on:mouseleave=end_drag
        >
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

            <section class="relative min-h-screen">
                <button
                    type="button"
                    class="absolute z-20 flex h-14 w-14 items-center justify-center rounded-2xl border border-custom-ring bg-custom-background/90 shadow-lg backdrop-blur transition-shadow hover:shadow-xl"
                    style=move || {
                        let (x, y) = position.get();
                        format!("left: {x}px; top: {y}px;")
                    }
                    on:mousedown=start_drag
                    on:click=toggle_popup
                >
                    <SyncMedTile badge_count=move || {
                        filled_count
                            .get()
                            .and_then(|res| res.ok())
                            .unwrap_or(0)
                    }/>
                </button>

                {move || if is_open.get() {
                    view! {
                        <div
                            class="absolute z-10"
                            style=move || {
                                let (x, y) = position.get();
                                format!("left: {}px; top: {}px;", x + 72, y)
                            }
                        >
                            <div class="rounded-[22px] border border-custom-ring bg-custom-subtle-background p-4 shadow-[0_16px_35px_rgba(0,0,0,0.16)] animate-[fadeIn_160ms_ease-out]">
                                <div class="mb-3 flex items-center gap-3">
                                    <SyncMedTile badge_count=move || {
                                        filled_count
                                            .get()
                                            .and_then(|res| res.ok())
                                            .unwrap_or(0)
                                    }/>
                                    <div>
                                        <p class="text-sm font-semibold text-custom-foreground">"SyncMed Widget"</p>
                                        <p class="text-xs text-custom-primary">"Select entry to continue"</p>
                                    </div>
                                </div>
                                <PatientListCard/>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </section>
        </main>
    }
}

#[component]
fn SyncMedTile(
    #[prop(into)] badge_count: Signal<i64>,
) -> impl IntoView {
    view! {
        <div class="relative flex h-12 w-12 items-center justify-center rounded-2xl border border-custom-ring/70 bg-custom-background/90 backdrop-blur">
            <div class="h-8 w-8 rounded-lg bg-custom-primary/85"></div>
            {move || {
                let value = badge_count.get();
                (value > 0).then(move || {
                    view! {
                        <span class="absolute -right-1 -top-1 inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-red-500 px-1 text-[10px] font-semibold text-white">
                            {value.to_string()}
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

#[server(GetFilledPatientCount, "/api")]
pub async fn get_filled_patient_count() -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{DbPool, schema::patient::dsl as patient_dsl};
        use axum::Extension;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel::dsl::count_star;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;

        let count = patient_dsl::patient
            .filter(patient_dsl::status.eq("filled"))
            .select(count_star())
            .first::<i64>(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("count query failed: {e}")))?;

        Ok(count)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_filled_patient_count is only available on the server".to_string(),
        ))
    }
}
