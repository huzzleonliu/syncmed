use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const MEDICINE_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/592defbd-fa4b-49d4-b550-c6a4ee112841";

#[component]
pub fn AppChatAccessibilityPage() -> impl IntoView {
    view! {
        <Title text="App Chat Accessibility - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
            <section class="mx-auto w-full max-w-[1326px] px-2 py-2 md:px-4 md:py-4">
                <div class="card overflow-hidden border border-custom-ring bg-custom-background shadow-md">
                    <div class="flex flex-col gap-3 border-b border-custom-border p-3 md:flex-row md:items-center md:justify-between md:p-4">
                        <h2 class="text-xl font-bold md:text-2xl">"Medication Reconciliation"</h2>
                        <div class="flex items-center gap-2 self-end md:self-auto">
                            <A href="/app/chat-default" attr:class="btn btn-primary btn-sm">"Switch to default page"</A>
                            <A href="/app/chat-default" attr:class="btn btn-primary btn-square btn-sm">"↗"</A>
                        </div>
                    </div>

                    <div class="bg-custom-subtle-background p-2 md:p-3">
                        <div class="rounded-lg bg-custom-background lg:grid lg:min-h-[620px] lg:grid-cols-2 lg:gap-6">
                            <div class="space-y-1 p-1 md:space-y-2 md:p-2 lg:space-y-2 lg:p-3">
                                <AccBubble
                                    text="Hello! I'm here to help you create a complete list of medications Huzz. Let's start by discussing any prescription medications you're currently taking. What medications do you take regularly?"
                                    time="10:19"
                                    user=false
                                    with_medicine=false
                                />
                                <AccBubble
                                    text="I'm using Paracet and ......"
                                    time="10:19"
                                    user=true
                                    with_medicine=false
                                />
                                <AccBubble
                                    text="Do you mean Paracet? If yes, please press “1”, if no, please press “2” and tell us the name of the medicine after “Beep”"
                                    time="10:19"
                                    user=false
                                    with_medicine=true
                                />
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
                                    <A href="/app/chat-confirm" attr:class="btn h-14 min-h-14 rounded-full border-none bg-custom-accent px-8 text-2xl font-medium text-custom-foreground hover:bg-custom-accent md:h-16 md:min-h-16 md:px-10 md:text-4xl">
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
    text: &'static str,
    time: &'static str,
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
