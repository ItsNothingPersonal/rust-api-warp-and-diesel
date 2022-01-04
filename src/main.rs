
itconfig::config! {
    PORT: u16 => 8000,
    database {
        URL < (
            "postgres://",
            POSTGRES_USERNAME => "postgres",
            ":",
            POSTGRES_PASSWORD: String,
            "@",
            POSTGRES_HOST => "localhost:5432",
            "/",
            POSTGRES_DB => "test",
        ),

        pool {
            MAX_OPEN: u64 => 32,
            MAX_IDLE: u64 => 8,
            TIMEOUT_SECONDS: u64 => 15,
        },
    },

    init_sql: String => "./db.sql",
    TABLE: String => "todo",
    SELECT_FIELDS: String => "id, name, created_at, checked"
}

fn main() {
    println!("Hello, world!");
}