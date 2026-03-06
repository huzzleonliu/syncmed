use leptos::prelude::*;

use leptos::task::spawn_local;
use leptos_router::components::A;

use leptos_meta::*;
use serde::{Serialize, Deserialize};
#[cfg(feature="ssr")]
use rand::prelude::*;


#[component]
pub fn MathHelper() -> impl IntoView {
    let (answer, set_answer) = signal("answer".to_string());
    let (vertifyrst, set_vertifyrst) = signal(Option::<bool>::None);
    let (showdetail, set_showdetail) = signal(false);
    // 使用Resource组件从服务器端生成问题
    let get_question = Resource::new(|| (), |_| generate_question());
    let use_question = Memo::new(move |_|{
        get_question.get()
            .and_then(|res| res.ok())
            .unwrap_or_else(||Question::new())
    });

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Mathematic practice for Thor"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-900 to-blue-100 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-col items-start flex-wrap m-auto w-xl">
                    //显示问题
                    {
                        move ||{
                            let q = use_question.get();
                            view!{
                                <p class="text-3xl my-3 w-full"> {q.x} * {q.y} </p>
                            }
                        }
                    }
                    // 使用daysiUI的input组件输入答案
                    <input type="number" 
                        class="input input-bordered w-full" 
                        on:input=move|event|{
                            set_answer.set(event_target_value(&event));
                        }
                    />
                    <button 
                        class="bg-blue-300 border-blue-800 my-3 p-2 rounded hover:bg-blue-500 hover:font-bold w-full"
                        on:click = move |_| {
                            get_question.refetch();
                            set_showdetail.set(false);
                    }> generate questions </button>
                    <button 
                        class="bg-blue-300 border-blue-800 my-3 p-2 rounded hover:bg-blue-500 hover:font-bold w-full"
                        on:click = move |_| {
                        let user_answer = answer.get_untracked();
                        let question = use_question.get_untracked();
                        let is_correct = user_answer.trim() == question.result.to_string();
                        set_vertifyrst.set(Some(is_correct));
                        
                            // 在服务器端插入数据，使用spawn_local异步任务插入数据，避免阻塞主线程
                            spawn_local(async move {
                                let _ = save_question_answer_to_db(question, user_answer).await;
                            });
                        
                    }> vertify answer</button>
                    {move ||{
                                match vertifyrst.get() {
                                    Some(true) => {
                                        view!{<p class="font-bold">"Congrats, the answer is right"</p>}.into_any()
                                    },
                                    Some(false) => {
                                        view!{<p class="font-bold">"try again or you can try to reveal the answer"</p>}.into_any()
                                    },
                                    None => view!{}.into_any()
                                }
                    }}
                    <button 
                        class="bg-blue-300 border-blue-800 my-3 p-2 rounded hover:bg-blue-500 hover:font-bold w-full"
                        on:click = move |_| {
                        if vertifyrst.get_untracked() != None{
                            set_showdetail.set(true);
                        }
                    }>reveal detail</button>
                    {move ||{
                                if showdetail.get() == true{
                                    view!{<DetailAnswer question=use_question.get()/>}.into_any()
                                } else {
                                    view!{}.into_any()
                                }
                    }}
                    
                    <A
                        href="/math-helper/history"
                        attr:class="btn btn-secondary my-3 w-full"
                    >
                        "View answer history"
                    </A>
                    
                </div>
            </div>
        </main>
    }
}


#[component]
pub fn MathHelperHistory() -> impl IntoView {
    let history = Resource::new(|| (), |_| get_question_log_from_db());

    view! {
        <Title text="Answer history"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-900 to-blue-100 text-white font-mono flex flex-col min-h-screen">
                <div class="card bg-base-100 text-base-content shadow-xl m-auto w-11/12 max-w-5xl">
                    <div class="card-body">
                        <div class="flex flex-wrap gap-2 justify-between items-center">
                            <h1 class="card-title text-2xl">"Answer history"</h1>
                            <div class="flex gap-2">
                                <button class="btn btn-outline btn-sm" on:click=move |_| history.refetch()>
                                    "Refresh"
                                </button>
                                <A href="/math-helper" attr:class="btn btn-primary btn-sm">
                                    "Back to practice"
                                </A>
                            </div>
                        </div>

                        {move || match history.get() {
                            None => view! {
                                <div class="w-full flex justify-center py-8">
                                    <span class="loading loading-spinner loading-lg"></span>
                                </div>
                            }.into_any(),
                            Some(Ok(items)) if items.is_empty() => view! {
                                <div role="alert" class="alert alert-info">
                                    <span>"No history yet. Verify an answer first."</span>
                                </div>
                            }.into_any(),
                            Some(Ok(items)) => view! {
                                <div class="overflow-x-auto">
                                    <table class="table table-zebra">
                                        <thead>
                                            <tr>
                                                <th>"ID"</th>
                                                <th>"Question"</th>
                                                <th>"Your answer"</th>
                                                <th>"Correct answer"</th>
                                                <th>"Result"</th>
                                                <th>"Created at"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {
                                                items
                                                    .into_iter()
                                                    .map(|item| {
                                                        let result_badge = if item.is_correct {
                                                            view! { <span class="badge badge-success">"Correct"</span> }.into_any()
                                                        } else {
                                                            view! { <span class="badge badge-error">"Wrong"</span> }.into_any()
                                                        };

                                                        view! {
                                                            <tr>
                                                                <td>{item.id}</td>
                                                                <td>{format!("{} * {}", item.x, item.y)}</td>
                                                                <td>{item.user_answer}</td>
                                                                <td>{item.correct_answer}</td>
                                                                <td>{result_badge}</td>
                                                                <td>{item.created_at}</td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        </tbody>
                                    </table>
                                </div>
                            }.into_any(),
                            Some(Err(err)) => view! {
                                <div role="alert" class="alert alert-error">
                                    <span>{format!("Failed to load history: {err}")}</span>
                                </div>
                            }.into_any(),
                        }}
                    </div>
                </div>
            </div>
        </main>
    }
}


// 定义结构体的时候因为是纯数据，所以不需要声明cfg是不是ssr
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Question{
    x:i32,
    y:i32,
    result:i32,
    x_1:i32,
    x_2:i32,
    x_3:i32,
    y_1:i32,
    y_2:i32,
    y_3:i32,
    y_1_m:i32,
    y_2_m:i32,
    y_3_m:i32
}

impl Question {
    fn new() -> Self{
        Self {
            x:0,
            y:0,
            result:0,
            x_1:0,
            x_2:0,
            x_3:0,
            y_1:0,
            y_2:0,
            y_3:0,
            y_1_m:0,
            y_2_m:0,
            y_3_m:0
        }
    }
}

#[component]
pub fn DetailAnswer (question:Question)->  impl IntoView{
    view!{
        <div class="bg-blue-100 text-5xl text-right my-8 p-5 flex flex-col w-full">
            <div class="flex flex-row justify-between items-end">
                <p class="text-blue-700">*</p>
                <div class="w-full flex flex-col justify-end">
                    <p class="text-blue-700 hover:font-bold">{question.x}</p>
                    <div class="justify-end flex flex-row">
                        <p class="text-orange-900 hover:font-bold">{question.y_3}</p>
                        <p class="text-orange-700 hover:font-bold">{question.y_2}</p>
                        <p class="text-orange-500 hover:font-bold">{question.y_1}</p>
                    </div>
                </div>
            </div>
            <p class="text-blue-900">"------------------"</p>
            <div class="flex flex-row justify-between items-end">
                <p class="text-blue-700">+</p>
                <div class="flex flex-col">
                    <p class="text-orange-500 hover:font-bold">{question.y_1_m}</p>
                    <p class="text-orange-700 hover:font-bold">{question.y_2_m}</p>
                    <p class="text-orange-900 hover:font-bold">{question.y_3_m}</p>
                </div>
            </div>
            <p class="text-blue-900">"------------------"</p>
            <p class="text-blue-500 hover:font-bold">{question.result}</p>
        </div>
    }

}

// 使用ssr在服务器端生成问题
#[server]
pub async fn generate_question() -> Result<Question, ServerFnError> {
    let mut result = Question::new();
    let mut rng = rand::rng();

    let mut nums: Vec<i32> = (100..1000).collect();
    nums.shuffle(&mut rng);
    result.x = nums.choose(&mut rng).expect("no number").clone();
    result.y = nums.choose(&mut rng).expect("no number").clone();
    result.result = result.x * result.y;

    let mut x = result.x;
    let mut y = result.y;

    result.x_1 = x % 10;
    x /= 10;
    result.x_2 = x % 10;
    x /= 10;
    result.x_3 = x % 10;

    result.y_1 = y % 10;
    y /= 10;
    result.y_2 = y % 10;
    y /= 10;
    result.y_3 = y % 10;

    result.y_1_m = result.x * result.y_1;
    result.y_2_m = result.x * result.y_2 * 10;
    result.y_3_m = result.x * result.y_3 * 100;

    Ok(result)
}


// 获取数据
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct QuestionLogItem {
    pub id: i32,
    pub x: String,
    pub y: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub is_correct: bool,
    pub created_at: String,
}

// 从数据库中查询数据的示例代码
#[server]
pub async fn get_question_log_from_db() -> Result<Vec<QuestionLogItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::schema::question_log;
        use crate::db::DbPool;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;

        let axum::Extension(pool): axum::Extension<DbPool> = leptos_axum::extract()
            .await
            .map_err(ServerFnError::new)?;

        let mut conn = pool
            .get()
            .await
            .map_err(ServerFnError::new)?;

        let logs = question_log::table
            .order(question_log::created_at.desc())
            .load::<crate::db::models::QuestionLog>(&mut conn)
            .await
            .map_err(ServerFnError::new)?;

        let items = logs
            .into_iter()
            .map(|log| QuestionLogItem {
                id: log.id,
                x: log.x,
                y: log.y,
                user_answer: log.user_answer,
                correct_answer: log.correct_answer,
                is_correct: log.is_correct,
                created_at: log.created_at.to_string(),
            })
            .collect();

        return Ok(items);
    }

    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}

// 在服务器端插入数据的示例代码
#[server]
pub async fn save_question_answer_to_db(
    question: Question,
    user_answer: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::models::NewQuestionLog;
        use crate::db::schema::question_log;
        use crate::db::DbPool;
        use diesel_async::RunQueryDsl;

        let axum::Extension(pool): axum::Extension<DbPool> = leptos_axum::extract()
            .await
            .map_err(ServerFnError::new)?;

        let mut conn = pool
            .get()
            .await
            .map_err(ServerFnError::new)?;

        let normalized_user_answer = user_answer.trim().to_string();
        let new_log = NewQuestionLog {
            x: question.x.to_string(),
            y: question.y.to_string(),
            user_answer: normalized_user_answer.clone(),
            correct_answer: question.result.to_string(),
            is_correct: normalized_user_answer == question.result.to_string(),
        };

        diesel::insert_into(question_log::table)
            .values(&new_log)
            .execute(&mut conn)
            .await
            .map_err(ServerFnError::new)?;

        return Ok(());
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (question, user_answer);
        Ok(())
    }
}
