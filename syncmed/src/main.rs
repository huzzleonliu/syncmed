#![recursion_limit = "256"]

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use syncmed_lib::app::{App, shell};

use axum::Extension;
use dotenvy::dotenv;
use std::env;
use syncmed_lib::db::create_pool;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();
    // 创建数据库连接池
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(database_url).await;

    // 创建Leptos配置
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    // 生成路由列表
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        // 把连接池作为状态 (State) 注入到 Axum 和 Leptos 中
        .layer(Extension(pool.clone())) // 这样你能在任何 axum 拦截器里拿到
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
