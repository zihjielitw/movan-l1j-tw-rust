use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::postgres::PgRow;
use sqlx::Row;
use crate::game_server::client_packets::c_server_version::CServerVersion;
use crate::l1database::L1Database;

#[derive(sqlx::FromRow, Debug)]
pub struct Account {
    /** 使用者帳號唯一 ID */
    pub account_id: i64,
    /** 使用者帳號 */
    pub acc: String,
    /** 使用者密碼 */
    pub pwd: String,
}

pub struct AccountService {
    /** 帳戶是否有效 (True 代表有效)- */
    pub is_valid: bool,
}

impl AccountService {

    pub fn new() -> AccountService {

        AccountService {
            is_valid: false,
        }
    }

    pub async fn get_account_data_by_account(account:String) -> Option<Account> {

        /*let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(L1Database::get_pool().await).await?;

        println!("======>{}", row.0);*/

        let account_data = sqlx::query("select * from accounts where login = $1")
            .bind(account)
            .map(|row: PgRow| Account {
                account_id: row.get("account_id"),
                acc: row.get("login"),
                pwd: row.get("password"),
            })
            .fetch_one(L1Database::get_pool().await).await;

        let data = match account_data {
            Ok(account) => {
                Some(account)
            }
            Err(reason) => {
                eprintln!("DB 連線失敗: {}", reason);
                None
            }
        };

        data
    }

    pub fn encode_password(&self, raw_password: String) -> String {

        let hashed = hash(raw_password, DEFAULT_COST);

        hashed.unwrap_or_else(|_| String::new())
    }

    pub fn validate_password(&mut self, raw_password: String, hash_password: String) -> bool {

        // 認證成功後如果再度認證就判斷為失敗
        if  self.is_valid {
            return false;
        }

        self.is_valid = verify(raw_password, &hash_password).unwrap_or(false);
        self.is_valid
    }
}