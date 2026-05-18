use actix_session::{SessionMiddleware, config::PersistentSession, storage::RedisSessionStore};
use actix_web::cookie::{Key, time::Duration};
use actix_web::{App, HttpServer, middleware, web};
use deadpool_redis::{Config, Runtime};
use mpsp_portal::{db, routers, utils};
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // initialize environment
    dotenv::dotenv().ok();

    // initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = db::init_pool(&db_url);

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");
    let redis_cfg = Config::from_url(redis_url);
    let redis_pool = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Redis pool");
    let redis_store = RedisSessionStore::new_pooled(redis_pool)
        .await
        .expect("Redis session store");
    let secret_key = Key::from(&utils::get_secret_key());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_name("auth_session".to_owned())
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                    .build(),
            )
            .configure(routers::config)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
