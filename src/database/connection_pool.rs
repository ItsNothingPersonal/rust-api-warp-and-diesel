use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could be created")
}