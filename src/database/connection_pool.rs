use diesel::{
    r2d2::{Builder, ConnectionManager, Pool},
    PgConnection,
};
use std::time::Duration;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn pg_pool(db_url: &str, max_size: u32, min_idle: u32, timeout_seconds: u64) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Builder::new()
        .max_size(max_size)
        .min_idle(Some(min_idle))
        .connection_timeout(Duration::from_secs(timeout_seconds))
        .build(manager)
        .expect("Postgres connection pool could be created")
}
