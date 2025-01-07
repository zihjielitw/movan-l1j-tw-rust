
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

    pub fn read_c(&mut self) -> u8 {
        let i = self.decrypt[self.off as usize] & 0xff;
        self.off = self.off + 1;
        i
    }

    pub fn read_s(&mut self) -> String {

        let length = self.decrypt.len();

        /*println!("self.decrypt.len(): {}", self.decrypt.len());
        println!("self.off: {}", self.off);
        println!("length: {}", length);*/

        match self.client_language {
            3 => {

                let cover_string = &self.decrypt[self.off as usize..length];
                let text = String::from_utf8(cover_string.to_owned()).unwrap();
                //println!("text: {}", text);
                let find_idx = text.find('\0').unwrap_or(0);
                let sub_text = text[..find_idx].to_string();
                //println!("read_s: {}", sub_text);
                self.off = self.off + sub_text.len() as i32 + 1;
                sub_text
            }
            _ => String::new()
        }
    }

}