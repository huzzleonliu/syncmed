pub mod models;
pub mod schema;

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
// 定义一个类型别名方便写
pub type DbPool = Pool<AsyncPgConnection>;
// 创建数据库连接池
pub async fn create_pool(database_url: String) -> DbPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .await
        .expect("Failed to create database pool")
}
