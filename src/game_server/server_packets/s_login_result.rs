use crate::game_server::server_base_packet::ServerBasePacket;
use crate::opcodes::Opcodes;

pub const REASON_LOGIN_OK: i32 = 0x00;
pub const REASON_ACCOUNT_ALREADY_EXISTS: i32 = 0x08;
pub const REASON_USER_OR_PASS_WRONG: i32 = 0x08;
pub const REASON_ACCESS_FAILED: i32 = 0x08;
pub const REASON_PASS_WRONG: i32 = 0x08;
pub const REASON_ACCOUNT_IN_USE: i32 = 0x16;
pub const REASON_OUT_OF_GASH: i32 = 0x16;

pub struct SLoginResult {
    reason: i32,
    server_base_packet: ServerBasePacket,
    tag: String,
}

impl SLoginResult {
    pub fn new(reason: i32) -> SLoginResult {
        let base_packet = ServerBasePacket::new();
        SLoginResult {
            reason,
            server_base_packet: base_packet,
            tag: String::from("[S] SLoginResult"),
        }
    }

    pub fn get_packets(&mut self) -> Vec<u8> {
        self.server_base_packet.write_c(Opcodes::SOpcodeLoginResult.value());
        self.server_base_packet.write_c(self.reason);
        self.server_base_packet.write_d(0x00000000);
        self.server_base_packet.write_d(0x00000000);
        self.server_base_packet.write_d(0x00000000);
        self.server_base_packet.write_d(0x00000000);
        self.server_base_packet.write_h(0x8c);
        self.server_base_packet.get_packets()
    }
}