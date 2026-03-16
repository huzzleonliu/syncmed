use leptos::prelude::*;
use leptos::ev::MouseEvent;
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

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
                class="pointer-events-none absolute top-0 left-1/2 w-[100vw] max-w-none -translate-x-1/2 border-t border-black/10 object-cover shadow-md"
            />

            <section class="relative min-h-screen">
                <button
                    type="button"
                    class="absolute z-10 flex h-14 w-14 items-center justify-center rounded-2xl border 
                    border-white/35 bg-white/6 shadow-[0_12px_28px_rgba(6,24,44,0.35)] backdrop-blur-sm 
                    backdrop-saturate-200 transition-shadow hover:shadow-[0_16px_34px_rgba(6,24,44,0.45)]"
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
                            <PatientListCard/>
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
        <div class="relative flex h-12 w-12 items-center justify-center rounded-2xl border 
        border-white/30 bg-custom-background/18 backdrop-blur-lg 
        backdrop-saturate-150 backdrop-contrast-125">
            <img src="/logo.svg" alt="SyncMed logo" class="h-10 w-10 object-contain"/>
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
    let overview = Resource::new(|| (), |_| get_widget_patient_overview());

    view! {
        <div class="w-[230px] rounded-[22px] border border-white/10 bg-custom-subtle-background/8
          backdrop-blur-xs p-4 shadow-[0_16px_35px_rgba(0,0,0,0.16)] animate-[fadeIn_160ms_ease-out]">
            <div class="mb-3 grid grid-cols-2 gap-2 text-center">
                <div class="rounded-xl border-2
                border-white/40 
                shadow-[0_20px_50px_rgba(0,0,0,0.3)]
                ring-1 ring-white/10
                bg-custom-background/30 backdrop-blur-lg backdrop-brightness-125 backdrop-contrast-125 
                py-2">
                    <p class="text-xs font-semibold text-custom-card-foreground">"New Entry"</p>
                    <p class="text-lg font-bold text-custom-foreground">
                        {move || {
                            overview
                                .get()
                                .and_then(|res| res.ok())
                                .map(|r| r.new_entry_count.to_string())
                                .unwrap_or_else(|| "0".to_string())
                        }}
                    </p>
                </div>
                <div class="rounded-xl border-2 border-white/40 
                shadow-[0_20px_50px_rgba(0,0,0,0.3)]
                ring-1 ring-white/10
                bg-custom-background/30 backdrop-blur-lg backdrop-brightness-125 backdrop-contrast-125 py-2">
                    <p class="text-xs font-semibold text-custom-card-foreground">"Total Entry"</p>
                    <p class="text-lg font-bold text-custom-foreground">
                        {move || {
                            overview
                                .get()
                                .and_then(|res| res.ok())
                                .map(|r| r.total_entry_count.to_string())
                                .unwrap_or_else(|| "0".to_string())
                        }}
                    </p>
                </div>
            </div>
            <div class="space-y-1.5">
                <Suspense fallback=move || view! {
                    <div class="rounded border border-custom-ring/40 bg-custom-background px-3 py-1 text-sm text-custom-card-foreground">
                        "Loading..."
                    </div>
                }>
                    {move || match overview.get() {
                        Some(Ok(data)) => {
                            if data.items.is_empty() {
                                view! {
                                    <div class="rounded border border-custom-ring/40 bg-custom-background px-3 py-1 text-sm text-custom-card-foreground">
                                        "No recent entries"
                                    </div>
                                }.into_any()
                            } else {
                                let (filled_items, other_items): (Vec<WidgetPatientRow>, Vec<WidgetPatientRow>) =
                                    data.items
                                        .into_iter()
                                        .partition(|item| item.status == "filled");

                                filled_items
                                    .into_iter()
                                    .map(|item| {
                                        view! {
                                            <A
                                                href=format!("/windows/card-details?patient-id={}", item.patient_key)
                                                attr:class="block rounded border border-custom-primary/40 bg-custom-secondary px-3 py-1 text-sm font-bold text-custom-foreground hover:bg-custom-secondary"
                                            >
                                                {format!("{}   {}   {}", item.name_snapshot, item.age_snapshot, item.gender_snapshot)}
                                            </A>
                                        }
                                    })
                                    .chain(
                                        other_items
                                            .into_iter()
                                            .map(|item| {
                                                view! {
                                                    <A
                                                        href=format!("/windows/card-details?patient-id={}", item.patient_key)
                                                        attr:class="block rounded border border-custom-primary/40 bg-custom-secondary px-3 py-1 text-sm font-normal text-custom-foreground hover:bg-custom-secondary"
                                                    >
                                                        {format!("{}   {}   {}", item.name_snapshot, item.age_snapshot, item.gender_snapshot)}
                                                    </A>
                                                }
                                            })
                                    )
                                    .collect_view()
                                    .into_any()
                            }
                        }
                        Some(Err(_)) => view! {
                            <div class="rounded border border-red-300 bg-red-50 px-3 py-1 text-sm text-red-600">
                                "Failed to load recent entries"
                            </div>
                        }.into_any(),
                        None => view! {
                            <div class="rounded border border-custom-ring/40 bg-custom-background px-3 py-1 text-sm text-custom-card-foreground">
                                "Loading..."
                            </div>
                        }.into_any(),
                    }}
                </Suspense>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WidgetPatientRow {
    pub patient_key: String,
    pub name_snapshot: String,
    pub age_snapshot: i32,
    pub gender_snapshot: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WidgetPatientOverview {
    pub new_entry_count: i64,
    pub total_entry_count: i64,
    pub items: Vec<WidgetPatientRow>,
}

#[server(GetWidgetPatientOverview, "/api")]
pub async fn get_widget_patient_overview() -> Result<WidgetPatientOverview, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{DbPool, models::PatientCase, schema::patient::dsl as patient_dsl};
        use axum::Extension;
        use diesel::dsl::count_star;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;

        let new_entry_count = patient_dsl::patient
            .filter(patient_dsl::status.eq("filled"))
            .select(count_star())
            .first::<i64>(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("new entry count query failed: {e}")))?;

        let total_entry_count = patient_dsl::patient
            .select(count_star())
            .first::<i64>(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("total count query failed: {e}")))?;

        let patients: Vec<PatientCase> = patient_dsl::patient
            .order((
                patient_dsl::modified_at.desc(),
                patient_dsl::requested_at.desc(),
            ))
            .limit(10)
            .load(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("recent query failed: {e}")))?;

        Ok(WidgetPatientOverview {
            new_entry_count,
            total_entry_count,
            items: patients
                .into_iter()
                .map(|p| WidgetPatientRow {
                    patient_key: p.patient_key,
                    name_snapshot: p.name_snapshot,
                    age_snapshot: p.age_snapshot,
                    gender_snapshot: p.gender_snapshot,
                    status: p.status,
                })
                .collect(),
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_widget_patient_overview is only available on the server".to_string(),
        ))
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
