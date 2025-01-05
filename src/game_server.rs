mod client_connection;
mod cipher;
mod client_base_packet;
mod client_packets;
mod server_base_packet;
mod server_packets;

use tracing::info;
use crate::game_context::GameContext;
use crate::game_server::client_connection::ClientConnection;

pub struct GameServer {
    pub host: String,
    pub port: u16,
}

impl GameServer {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        GameServer {
            host: host.into(),
            port,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
        info!("伺服器成功建立在 {}:{}", self.host, self.port);

        let global_context = GameContext::get_context().unwrap();
        info!("伺服器啟動時間 {}", global_context.game_server_start_time);

        info!("等待客戶端連接中...");
        loop  {
            let (socket, addr) = listener.accept().await?;
            println!("新客戶端連線 {}", addr);

            tokio::task::spawn(async move {
                let mut client_connection = ClientConnection::new(socket);
                client_connection.handle_packet().await;
            });
        }
    }
}