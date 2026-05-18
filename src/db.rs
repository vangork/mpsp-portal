use diesel_async::AsyncMysqlConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool};

pub type Connection = AsyncMysqlConnection;

pub type Pool = deadpool::Pool<AsyncMysqlConnection>;

pub fn init_pool(url: &str) -> Pool {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncMysqlConnection>::new(url);
    deadpool::Pool::builder(config)
        .build()
        .expect("Failed to create pool.")
}
