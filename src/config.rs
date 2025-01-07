use std::fs;
use std::process::exit;
use serde::Deserialize;
use tokio::sync::OnceCell;


static GLOBAL_GAME_CONFIG: OnceCell<GameConfig> = OnceCell::const_new();
static GLOBAL_SERVER_CONFIG: OnceCell<ServerConfig> = OnceCell::const_new();


#[derive(Deserialize)]
pub struct GameConfig {
    pub game: ConfigGameNode,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub server: ConfigServerNode,
    pub database: ConfigDatabaseNode,
}

#[derive(Deserialize)]
pub struct ConfigGameNode {
    pub global_chat_level: i32,
}

#[derive(Deserialize)]
pub struct ConfigServerNode {
    pub allow_multiple_pc: bool,
    pub client_language: i32,
    pub hostname: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct ConfigDatabaseNode {
    pub db_account: String,
    pub db_host: String,
    pub db_name: String,
    pub db_password: String,
    pub db_pool_max_connections: u32,
    pub db_port: u16,
}

impl GameConfig {
    pub async fn init_config() {
        GLOBAL_GAME_CONFIG.get_or_init(|| async {
            Self::new()
        }).await;
    }

    pub fn get_config() -> Option<&'static GameConfig> {
        GLOBAL_GAME_CONFIG.get()
    }

    fn new() -> Self {

        let game_config = match fs::read_to_string("./config/config.toml") {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read config file ./config/config.toml");
                exit(1);
            }
        };

        let config_data: GameConfig = match toml::from_str(&game_config) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("./config/config.toml 缺少必要的參數，無法繼續");
                exit(1);
            }
        };

        config_data
    }
}

impl ServerConfig {
    pub async fn init_config() {
        GLOBAL_SERVER_CONFIG.get_or_init(|| async {
            Self::new()
        }).await;
    }

    pub fn get_config() -> Option<&'static ServerConfig> {
        GLOBAL_SERVER_CONFIG.get()
    }

    fn new() -> Self {

        let server_config = match fs::read_to_string("./config/server.toml") {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read config file ./config/server.toml");
                exit(1);
            }
        };

        let config_data: ServerConfig = match toml::from_str(&server_config) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("./config/server.toml 缺少必要的參數，無法繼續");
                exit(1);
            }
        };

        config_data
    }
}