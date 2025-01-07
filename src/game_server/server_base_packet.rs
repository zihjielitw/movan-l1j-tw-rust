use bytes::{BufMut, BytesMut};
use crate::config::ServerConfig;

pub struct ServerBasePacket {
   buffer: BytesMut,
    client_language: i32,
}

impl ServerBasePacket {
    pub fn new() -> ServerBasePacket {
        let buffer = BytesMut::new();

        let global_server_config = ServerConfig::get_config().unwrap();
        let client_language =  global_server_config.server.client_language;

        ServerBasePacket {
            buffer,
            client_language
        }
    }

    pub fn write_d(&mut self, value: i32) {
        self.buffer.put_u8((value & 0xff) as u8);
        self.buffer.put_u8((value >> 8 & 0xff) as u8);
        self.buffer.put_u8((value >> 16 & 0xff) as u8);
        self.buffer.put_u8((value >> 24 & 0xff) as u8);
    }

    pub fn write_h(&mut self, value: i32) {
        self.buffer.put_u8((value & 0xff) as u8);
        self.buffer.put_u8((value >> 8 & 0xff) as u8);
    }

    pub fn write_c(&mut self, value: i32) {
        self.buffer.put_u8((value & 0xff) as u8);
    }

    pub fn write_p(&mut self, value: i32) {
        self.buffer.put_u8(value as u8);
    }

    pub fn write_l(&mut self, value: i64) {
        self.buffer.put_u8(value as u8);
    }

    pub fn write_exp(&mut self, value: i64) {
        self.buffer.put_u8(((value & 0xff) as i32) as u8 );
        self.buffer.put_u8(((value >> 8 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((value >> 16 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((value >> 24 & 0xff) as i32) as u8 );
    }

    pub fn write_f(&mut self, value: f64) {
        let temp_val = value as i64;

        self.buffer.put_u8(((temp_val & 0xff) as i32) as u8 );
        self.buffer.put_u8(((temp_val >> 8 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((temp_val >> 16 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((temp_val >> 24 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((temp_val >> 32 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((temp_val >> 40 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((temp_val >> 48 & 0xff) as i32) as u8 );
        self.buffer.put_u8(((temp_val >> 56 & 0xff) as i32) as u8 );
    }

    pub fn write_s(&mut self, text: String) {
        if text.len() > 0 {

            match self.client_language {
                3 => {
                   /* let cover_string = self.encode_big5(text.into_bytes());
                    self.buffer.put(cover_string.as_bytes());*/
                }
                _ => ()
            }
        }

        self.buffer.put_u8(0x00);
    }

    pub fn write_byte(&mut self, value: Vec<u8>) {
        self.buffer.put(value.as_slice());
    }

    pub fn get_packets(&mut self) -> Vec<u8> {
       let padding = self.buffer.len() % 4;
        if padding != 0 {
            for _ in padding..4 {
                self.write_c(0)
            }
        }



        let final_packet = self.buffer.to_vec();
        self.buffer.clear();
        final_packet
    }
}