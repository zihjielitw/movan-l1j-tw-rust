use crate::game_server::client_base_packet::ClientBasePacket;
use crate::game_server::server_packets::s_server_version::SServerVersion;

pub struct CServerVersion {
    client_base_packet: ClientBasePacket,
    current_client_version: i32,
    tag: String,
}

impl CServerVersion {
    pub fn new(decrypt: Vec<u8>) -> CServerVersion {
        let base_packet = ClientBasePacket::new(decrypt);

        CServerVersion {
            client_base_packet: base_packet,
            current_client_version: 0x00000000,
            tag: String::from("[C] CServerVersion"),
        }
    }

    pub fn get_packets(&self) -> Vec<u8> {
        /* From client: client version
         * [Client] opcode = 14
         * 0000: 0e 34 00/ b6 /03 00 00 00 /09 f0 6e f0 65 51 c7 00 .4........n.eQ..
         * 0010: 01 00 06 00 ....
         */
        /*readH();
        readC();
        int clientLanguage = readD();   // 主程式語系
        int unknownVer1 = readH();      // 未知的版本號
        int unknownVer2 = readH();      // 未知的版本號
        int clientVersion = readD();    // 主程式版本號
        */
        let mut server_version = SServerVersion::new();
        server_version.get_packets()
    }
}
