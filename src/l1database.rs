use std::process::exit;
use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;
use tracing::info;

static L1DATABASE: OnceCell<L1Database> = OnceCell::const_new();

pub struct L1Database {
    pool: Pool<Postgres>,
}

impl L1Database {
    pub async fn connection(host: String, port: u16, account: String, password: String, db_name: String, pool_max_connections:u32) -> &'static L1Database{
        L1DATABASE.get_or_init(|| async {
            Self::new(host, port, account, password, db_name, pool_max_connections).await
        }).await
    }

    pub async fn get_pool() -> &'static Pool<Postgres> {
        &L1DATABASE.get().unwrap().pool
    }

    async fn new(host: String, port: u16, account: String, password: String, db_name: String, pool_max_connections:u32) -> Self  {
        let pool = PgPoolOptions::new()
            .max_connections(pool_max_connections)
            .acquire_timeout(Duration::from_secs(10))
            .connect(format!("postgres://{}:{}@{}:{}/{}",account, password, host, port, db_name).as_str()).await;

      let db =  match pool {
            Ok(pool) => {
                info!("DB 連線成功");
                L1Database { pool }
            }
            Err(reason) => {
                eprintln!("DB 連線失敗: {}", reason);
                exit(1);
            }
        };
        db
    }
}