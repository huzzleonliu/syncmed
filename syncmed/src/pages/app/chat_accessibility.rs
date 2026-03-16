use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};
use crate::structure::chat::CaseChatMessageDraft;
#[cfg(target_arch = "wasm32")]
use crate::structure::chat::{ChatbotRequest, ChatbotResponse};

const MEDICINE_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/592defbd-fa4b-49d4-b550-c6a4ee112841";

#[component]
pub fn AppChatAccessibilityPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });
    let (draft, set_draft) = signal(String::new());
    let (messages, set_messages) = signal(Vec::<CaseChatMessageDraft>::new());
    let patient_key_for_send = patient_key.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &patient_key_for_send;

    view! {
        <Title text="App Chat Accessibility - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
            <section class="mx-auto w-full max-w-[1326px] px-2 py-2 md:px-4 md:py-4">
                <div class="card overflow-hidden border border-custom-ring bg-custom-background shadow-md">
                    <div class="flex flex-col gap-3 border-b border-custom-border p-3 md:flex-row md:items-center md:justify-between md:p-4">
                        <h2 class="text-xl font-bold md:text-2xl">"Medication Reconciliation"</h2>
                        <div class="flex items-center gap-2 self-end md:self-auto">
                            <A
                                href=move || {
                                    let key = patient_key.get();
                                    if key.trim().is_empty() {
                                        "/app/chat-default".to_string()
                                    } else {
                                        format!("/app/chat-default?patient-id={key}")
                                    }
                                }
                                attr:class="btn btn-primary btn-sm"
                            >
                                "Switch to default page"
                            </A>
                            <A
                                href=move || {
                                    let key = patient_key.get();
                                    if key.trim().is_empty() {
                                        "/app/chat-default".to_string()
                                    } else {
                                        format!("/app/chat-default?patient-id={key}")
                                    }
                                }
                                attr:class="btn btn-primary btn-square btn-sm"
                            >
                                "↗"
                            </A>
                        </div>
                    </div>

                    <div class="bg-custom-subtle-background p-2 md:p-3">
                        <div class="rounded-lg bg-custom-background lg:grid lg:min-h-[620px] lg:grid-cols-2 lg:gap-6">
                            <div class="space-y-1 p-1 md:space-y-2 md:p-2 lg:space-y-2 lg:p-3">
                                {move || {
                                    let items = messages.get();
                                    if items.is_empty() {
                                        view! { <p class="text-sm text-custom-muted-foreground">"No chat messages yet."</p> }.into_any()
                                    } else {
                                        items
                                            .into_iter()
                                            .enumerate()
                                            .map(|(idx, item)| {
                                                view! {
                                                    <AccBubble
                                                        text=item.content_text
                                                        time=item.created_at
                                                        user=item.sender_type == "patient"
                                                        with_medicine=idx == 0 && item.sender_type != "patient"
                                                    />
                                                }
                                            })
                                            .collect_view()
                                            .into_any()
                                    }
                                }}
                            </div>

                            <div class="mt-2 border-t border-custom-border p-3 md:p-4 lg:mt-0 lg:border-l lg:border-t-0 lg:p-6">
                                <div class="mx-auto grid w-fit grid-cols-3 gap-x-3 gap-y-2 md:gap-x-8">
                                    <KeyPadButton main="1" sub=""/>
                                    <KeyPadButton main="2" sub="ABC"/>
                                    <KeyPadButton main="3" sub="DEF"/>
                                    <KeyPadButton main="4" sub="GHI"/>
                                    <KeyPadButton main="5" sub="JKL"/>
                                    <KeyPadButton main="6" sub="MNO"/>
                                    <KeyPadButton main="7" sub="PQRS"/>
                                    <KeyPadButton main="8" sub="TUV"/>
                                    <KeyPadButton main="9" sub="WXYZ"/>
                                    <KeyPadButton main="*" sub=""/>
                                    <KeyPadButton main="0" sub="+"/>
                                    <KeyPadButton main="#" sub=""/>
                                </div>

                                <div class="mt-3 flex justify-center lg:mt-10">
                                    <div class="mr-2 flex-1">
                                        <input
                                            type="text"
                                            class="input input-bordered w-full border-custom-input bg-custom-background text-custom-foreground"
                                            placeholder="Type message..."
                                            prop:value=move || draft.get()
                                            on:input=move |ev| set_draft.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        on:click=move |_| {
                                            let text = draft.get_untracked().trim().to_string();
                                            if text.is_empty() {
                                                return;
                                            }
                                            set_draft.set(String::new());
                                            set_messages.update(|items| {
                                                items.push(CaseChatMessageDraft {
                                                    id: None,
                                                    patient_id: None,
                                                    sender_type: "patient".to_string(),
                                                    content_text: text.clone(),
                                                    created_at: "Now".to_string(),
                                                });
                                            });
                                            #[cfg(target_arch = "wasm32")]
                                            {
                                                let set_messages = set_messages;
                                                let key = patient_key_for_send.get_untracked();
                                                leptos::task::spawn_local(async move {
                                                    let payload = ChatbotRequest {
                                                        message: text,
                                                        patient_id: if key.trim().is_empty() {
                                                            None
                                                        } else {
                                                            Some(key)
                                                        },
                                                    };
                                                    let request = gloo_net::http::Request::post(
                                                        "http://chatbot:3003/api/chat",
                                                    )
                                                    .header("content-type", "application/json")
                                                    .json(&payload);

                                                    match request {
                                                        Ok(req) => match req.send().await {
                                                            Ok(resp) => {
                                                                match resp
                                                                    .json::<ChatbotResponse>()
                                                                    .await
                                                                {
                                                                    Ok(data) => {
                                                                        set_messages.update(
                                                                            |items| {
                                                                                items.push(
                                                                                    CaseChatMessageDraft {
                                                                                        id: None,
                                                                                        patient_id: None,
                                                                                        sender_type: "bot"
                                                                                            .to_string(),
                                                                                        content_text: data.reply,
                                                                                        created_at: data.timestamp,
                                                                                    },
                                                                                );
                                                                            },
                                                                        );
                                                                    }
                                                                    Err(err) => {
                                                                        set_messages.update(
                                                                            |items| {
                                                                                items.push(
                                                                                    CaseChatMessageDraft {
                                                                                        id: None,
                                                                                        patient_id: None,
                                                                                        sender_type: "bot"
                                                                                            .to_string(),
                                                                                        content_text: format!(
                                                                                            "机器人响应解析失败: {err}"
                                                                                        ),
                                                                                        created_at: "Now".to_string(),
                                                                                    },
                                                                                );
                                                                            },
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                set_messages.update(|items| {
                                                                    items.push(
                                                                        CaseChatMessageDraft {
                                                                            id: None,
                                                                            patient_id: None,
                                                                            sender_type: "bot"
                                                                                .to_string(),
                                                                            content_text: format!(
                                                                                "请求机器人失败: {err}"
                                                                            ),
                                                                            created_at: "Now"
                                                                                .to_string(),
                                                                        },
                                                                    );
                                                                });
                                                            }
                                                        },
                                                        Err(err) => {
                                                            set_messages.update(|items| {
                                                                items.push(CaseChatMessageDraft {
                                                                    id: None,
                                                                    patient_id: None,
                                                                    sender_type: "bot".to_string(),
                                                                    content_text: format!(
                                                                        "构建请求失败: {err}"
                                                                    ),
                                                                    created_at: "Now".to_string(),
                                                                });
                                                            });
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    >
                                        "Send"
                                    </button>
                                    <A
                                        href=move || {
                                            let key = patient_key.get();
                                            if key.trim().is_empty() {
                                                "/app/chat-confirm".to_string()
                                            } else {
                                                format!("/app/chat-confirm?patient-id={key}")
                                            }
                                        }
                                        attr:class="btn h-14 min-h-14 rounded-full border-none bg-custom-accent px-8 text-2xl font-medium text-custom-foreground hover:bg-custom-accent md:h-16 md:min-h-16 md:px-10 md:text-4xl"
                                    >
                                        "End Chat"
                                    </A>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
fn AccBubble(
    text: String,
    time: String,
    user: bool,
    with_medicine: bool,
) -> impl IntoView {
    let bubble_class = if user {
        "bg-custom-input"
    } else {
        "bg-base-300"
    };
    view! {
        <div class=move || format!("rounded-lg border border-custom-border px-4 py-3 md:px-7 md:py-4 lg:max-w-[560px] lg:px-10 {}", bubble_class)>
            <p class="text-lg leading-[1.35] md:text-2xl">{text}</p>
            {move || if with_medicine {
                view! {
                    <div class="mt-3 flex items-start gap-4">
                        <img src=MEDICINE_IMAGE_URL alt="Medicine preview" class="h-[64px] w-[58px] object-cover md:h-[78px] md:w-[70px]"/>
                        <div>
                            <p class="text-2xl font-bold text-custom-foreground md:text-4xl lg:text-5xl">"medicine name"</p>
                            <p class="text-xl font-light text-custom-primary md:text-3xl lg:text-4xl">"company name"</p>
                            <p class="text-lg text-custom-primary md:text-2xl lg:text-4xl">"Function : Fever reducer, pain reliever"</p>
                        </div>
                    </div>
                }
                    .into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            <p class="mt-2 text-right text-sm text-custom-muted-foreground">{time}</p>
        </div>
    }
}

#[component]
fn KeyPadButton(main: &'static str, sub: &'static str) -> impl IntoView {
    view! {
        <button type="button" class="btn h-14 min-h-14 w-14 rounded-full border-none bg-custom-accent p-0 text-custom-foreground hover:bg-custom-accent md:h-20 md:min-h-20 md:w-20 lg:h-[90px] lg:min-h-[90px] lg:w-[90px]">
            <span class="flex flex-col items-center leading-none">
                <span class="text-xl md:text-3xl lg:text-4xl">{main}</span>
                <span class="text-[10px] md:text-xs">{sub}</span>
            </span>
        </button>
    }
}
