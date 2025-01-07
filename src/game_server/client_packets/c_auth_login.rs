use tracing::{info};
use crate::config::ServerConfig;
use crate::game_server::client_base_packet::ClientBasePacket;
use crate::game_server::server_packets::s_login_result::{SLoginResult, REASON_USER_OR_PASS_WRONG};
use crate::game_server::services::account_service::AccountService;

pub struct CAuthLogin {
    client_base_packet: ClientBasePacket,
    ip: String,
    tag: String,
}

impl CAuthLogin {
    pub fn new(decrypt: Vec<u8>, ip: String) -> CAuthLogin {
        let base_packet = ClientBasePacket::new(decrypt);
        CAuthLogin{
            client_base_packet: base_packet,
            ip: ip,
            tag: String::from("[C] CAuthLogin"),
        }
    }

    pub async fn get_packets(&mut self) -> Vec<u8> {
        let action = self.client_base_packet.read_c();
        let mut response_packets: Vec<u8> = vec![];

        println!("action ==> {:?}", action);

        match action {
            0x06 => 'login_request: {

                let account = self.client_base_packet.read_s().to_lowercase();
                let password = self.client_base_packet.read_s();

                info!("收到 {} 登入請求", account);

                println!("account:{}", account);
                println!("password:{}", password);

                let global_server_config = ServerConfig::get_config().unwrap();
                if !global_server_config.server.allow_multiple_pc {
                    // TODO 多開登入限制實作
                }


                let account_data = AccountService::get_account_data_by_account(account).await;
                println!("{:?}",account_data);

                if account_data.is_none() {
                    // TODO 實作是否自動建立帳號判斷
                    /*let mut s_login_result = SLoginResult::new(REASON_USER_OR_PASS_WRONG);
                    response_packets = s_login_result.get_packets();*/
                    break 'login_request
                }



            }
            _ => ()
        }

        response_packets
    }
}