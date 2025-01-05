mod config;
mod log;
mod game_server;
mod opcodes;
mod utils;
mod game_context;

use std::str;
use crate::config::{GameConfig, ServerConfig};
use crate::game_context::GameContext;
use crate::log::ServerLog;
use crate::game_server::GameServer;

#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    let guard = ServerLog::init_log();
    GameContext::init().await;
    GameConfig::init_config().await;
    ServerConfig::init_config().await;

    //let global_context = GameContext::get_context().unwrap();
   // let global_config = GameConfig::get_config().unwrap();
    let global_server_config = ServerConfig::get_config().unwrap();

    let mut game_server = GameServer::new(global_server_config.server.hostname.clone(), global_server_config.server.port.clone());
    game_server.run().await.expect("遊戲伺服器啟動失敗");

    drop(guard);
    Ok(())
}
