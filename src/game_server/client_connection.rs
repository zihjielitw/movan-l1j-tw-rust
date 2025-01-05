use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use crate::game_server::cipher::Cipher;
use crate::opcodes::Opcodes;

pub struct ClientConnection {
    active_char: bool,
    addr: String,
    cipher: Cipher,
    /*
    _login_status: i32,
    _switch: i32,
    _packet_handler: PacketHandler,*/
    //_stream: TcpStream,
    first_packet: [u8; 11],
    key: i32,
    key_hex: String,
    tcp_in: OwnedReadHalf,
    tcp_out: OwnedWriteHalf,
}


impl ClientConnection {
    pub fn new(stream: TcpStream) -> ClientConnection {
        let addr = stream.peer_addr().unwrap().to_string();
        let (read, write) = stream.into_split();
        let key_hex = format!("{:02X}", 12345);
        let key = i32::from_str_radix(key_hex.as_str(), 16).unwrap();
        let cipher = Cipher::new(key);
        ClientConnection {
            active_char:false,
            addr,
            cipher,
            first_packet:  [0x9d, 0xd1, 0xd6, 0x7a, 0xf4, 0x62, 0xe7, 0xa0, 0x66, 0x02, 0xfa],
            key,
            key_hex,
            tcp_in: read,
            tcp_out: write,
        }
    }

    pub async fn handle_packet(&mut self) {

        let bogus = (self.first_packet.len() + 7) as u8;

        let mut buffer = BytesMut::new();
        buffer.put_u8(bogus & 0xFF);
        buffer.put_u8(bogus.checked_shr(8).unwrap_or(0) & 0xFF);
        buffer.put_u8(Opcodes::SOpcodeInitPacket.value() as u8);
        buffer.put_u8( (self.key & 0xFF ) as u8);
        buffer.put_u8( (self.key  >> 8 & 0xFF) as u8 );
        buffer.put_u8((self.key  >> 16 & 0xFF) as u8 );
        buffer.put_u8((self.key  >> 24 & 0xFF) as u8 );
        buffer.put(&self.first_packet[..]);
        self.tcp_out.write_all(&buffer.freeze()).await.expect(format!("異常用戶端 ({}) 連結到伺服器, 已中斷該連線。", self.addr).as_str());
        println!("送出封包=======>");



        loop {
            let data = self.read_packet().await;
            let len = data.len();

            if len > 0 {

                let opcode = (data.first().unwrap() & 0xFF) as i32;

                match opcode {
                    // 處理多重登入
                    code if code == Opcodes::COpcodeBeanfunLoginPacket as i32  => {
                        println!("opcode=========>COpcodeBeanfunLoginPacket")
                    },
                    code if code == Opcodes::COpcodeChangeChar as i32  => {
                        println!("opcode=========>COpcodeChangeChar")
                    },
                    code if code == Opcodes::COpcodeLoginToServer as i32  => {
                        println!("opcode=========>COpcodeLoginToServer")
                    },
                    code if code == Opcodes::COpcodeLoginToServerOk as i32  => {
                        println!("opcode=========>COpcodeLoginToServerOk")
                    },
                    code if code == Opcodes::COpcodeClientVersion as i32  => {
                        println!("opcode=========>COpcodeClientVersion")
                    },
                    _ => println!("opcode=========>{}", opcode),
                }



            }
        }
    }

    async fn read_packet(&mut self) -> Bytes {

        let mut buf = [0u8; 128];

        match self.tcp_in.read(&mut buf).await {
            Ok(0) => {
                BytesMut::with_capacity(0).freeze()
            }
            Ok(n) => {
                println!("read_packet: {}", n);
                println!("客戶端加密封包: {:02X?}", &buf[2..n]);

                self.cipher.decrypt(&mut buf[2..n]);

                let mut bytes = BytesMut::with_capacity(n - 2);
                bytes.put(&buf[2..n]);
                bytes.freeze()
            }
            Err(e) => {
                println!("read error! {:?}", e);
                BytesMut::with_capacity(0).freeze()
            }
        }


    }
}