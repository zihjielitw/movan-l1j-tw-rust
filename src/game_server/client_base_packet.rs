
use encoding::all::BIG5_2003;
use encoding::{DecoderTrap, Encoding};
use crate::config::ServerConfig;

pub struct ClientBasePacket {
    client_language: i32,
    decrypt: Vec<u8>,
    off: i32
}

impl ClientBasePacket {
    pub fn new(decrypt: Vec<u8>) -> ClientBasePacket {

        let global_server_config = ServerConfig::get_config().unwrap();
        let client_language =  global_server_config.server.client_language;

        ClientBasePacket {
            client_language: client_language,
            decrypt:decrypt,
            off: 1
        }
    }

    pub fn read_c(&mut self) -> i32 {
        self.off += 1;
        let i = self.decrypt[self.off as usize] & 0xff;
        i as i32
    }

    pub fn read_s(&mut self) -> String {

        let length = self.decrypt.len() -  self.off as usize;

        match self.client_language {
            3 => {
                let mut cover_string = self.encode_big5(self.decrypt[self.off as usize..length].to_owned());
                cover_string = cover_string[..cover_string.find("\0").unwrap()].to_string();
                self.off += cover_string.clone().into_bytes().len() as i32 + 1;
                println!("read_s: {}", cover_string);
                cover_string
            }
            _ => String::new()
        }
    }

    fn encode_big5(&mut self, s: Vec<u8> ) -> String {
        BIG5_2003.decode(&s,DecoderTrap::Strict).unwrap()
    }
}