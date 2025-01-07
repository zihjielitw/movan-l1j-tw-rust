use crate::config::ServerConfig;
use crate::game_context::GameContext;
use crate::game_server::server_base_packet::ServerBasePacket;
use crate::opcodes::Opcodes;
use crate::utils;

pub struct SServerVersion {
    server_base_packet: ServerBasePacket,
    server_no: i32,
    tag: String,
}

impl SServerVersion {

    pub fn new() -> SServerVersion {
        let base_packet = ServerBasePacket::new();

        SServerVersion {
            server_base_packet: base_packet,
            server_no: 1,
            tag: String::from("[S] SServerVersion"),
        }
    }

    pub fn get_packets(&mut self) -> Vec<u8> {

        let global_context = GameContext::get_context().unwrap();
        let global_server_config = ServerConfig::get_config().unwrap();
        let client_language =  global_server_config.server.client_language;

        self.server_base_packet.write_c(Opcodes::SOpcodeServerVersion.value());
        self.server_base_packet.write_c(0);       // Auth ok?
        self.server_base_packet.write_c(self.server_no);    // Server Id
        self.server_base_packet.write_d(utils::hex::hex_to_decimal("07cbf4dd").unwrap_or(0) as i32);   // server version 3.80C Taiwan Server
        self.server_base_packet.write_d(utils::hex::hex_to_decimal("07cbf4dd").unwrap_or(0) as i32);   // cache version 3.80C Taiwan Server
        self.server_base_packet.write_d(utils::hex::hex_to_decimal("77fc692d").unwrap_or(0) as i32);   // auth version 3.80C Taiwan Server
        self.server_base_packet.write_d(utils::hex::hex_to_decimal("07cbf4d9").unwrap_or(0) as i32);   // npc version 3.80C Taiwan Server
        self.server_base_packet.write_d(global_context.game_server_start_timestamp);
        //self.server_base_packet.write_d(1712131415);
        self.server_base_packet.write_c(0);      // unknown
        self.server_base_packet.write_c(0);      // unknown
        self.server_base_packet.write_c(client_language);     // Country: 0.US 3.Taiwan 4.Japan 5.China
        self.server_base_packet.write_d(utils::hex::hex_to_decimal("087f7dc2").unwrap_or(0) as i32);    // Server Type
        self.server_base_packet.write_d((chrono::offset::Local::now().timestamp_millis() / 1000) as i32);       // Uptime
        //self.server_base_packet.write_d((1712131415) as i32);       // Uptime
        self.server_base_packet.write_h(1);

        self.server_base_packet.get_packets()
    }
}