use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use crate::game_server::cipher::Cipher;
use crate::game_server::client_packets::c_auth_login::CAuthLogin;
use crate::game_server::client_packets::c_server_version::CServerVersion;
use crate::opcodes::Opcodes;

pub struct ClientThread {
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


impl ClientThread {
    pub fn new(stream: TcpStream) -> ClientThread {
        let addr = stream.peer_addr().unwrap().to_string();
        let (read, write) = stream.into_split();
        let key_hex = format!("{:02X}", 12345);
        let key = i32::from_str_radix(key_hex.as_str(), 16).unwrap();
        let cipher = Cipher::new(key);
        ClientThread {
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

        let buf = [0u8; 128];

        loop {
            let data = self.read_packet(buf.to_vec()).await;
            let len = data.len();

            if len > 0 {

                let opcode = (data[0] & 0xFF) as i32;

                match opcode {
                    // 處理多重登入
                    code if code == Opcodes::COpcodeBeanfunLoginPacket.value()  => {
                        println!("opcode:{} , COpcodeBeanfunLoginPacket", code);

                        if !self.active_char {
                            let mut c_auth_login = CAuthLogin::new(data, self.addr[..self.addr.find(":").unwrap_or(0)].to_string());
                            let packets = c_auth_login.get_packets().await;

                            if packets.len() > 0 {
                                println!("發送封包至客戶端: {:02X?}", &packets[..]);
                                self.send_packet(packets).await;
                            }
                        }

                    },
                    code if code == Opcodes::COpcodeChangeChar.value()  => {
                        println!("opcode:{} , COpcodeChangeChar", code);
                    },
                    code if code == Opcodes::COpcodeLoginToServer.value()  => {
                        println!("opcode:{} , COpcodeLoginToServer", code);
                    },
                    code if code == Opcodes::COpcodeLoginToServerOk.value()  => {
                        println!("opcode:{} , COpcodeLoginToServerOk", code);
                    },
                    code if code == Opcodes::COpcodeClientVersion.value()  => {
                        println!("opcode:{} , COpcodeClientVersion", code);

                        if !self.active_char {
                            let c_server_version = CServerVersion::new();
                            let packets = c_server_version.get_packets();

                            if packets.len() > 0 {
                                println!("發送封包至客戶端: {:02X?}", &packets[..]);
                                self.send_packet(packets).await;
                            }

                        }
                    },
                    _ => {
                        println!("opcode=========>{}", opcode);
                    }
                }



            }
        }
    }

    async fn read_packet(&mut self, mut buf: Vec<u8>) -> Vec<u8> {



        match self.tcp_in.read(&mut buf).await {
            Ok(0) => {
                //Bytes::from(BytesMut::new())
               // BytesMut::with_capacity(0).freeze()
                Vec::new()
            }
            Ok(n) => {
                println!("read_packet: {}", n);
                println!("客戶端加密封包: {:02X?}", &buf[2..n]);

                let decrypt_data = self.cipher.decrypt(&mut buf[2..n]);

                let mut bytes = BytesMut::new();
                bytes.put(&decrypt_data[..]);
                bytes.to_vec()
            }
            Err(e) => {
                println!("read error! {:?}", e);
                Vec::new()
            }
        }


    }

    async fn send_packet(&mut self, mut packet: Vec<u8>) {

            let encrypt_data = self.cipher.encrypt(&mut *packet);

            let length = encrypt_data.len() + 2;

            let mut buffer = BytesMut::new();
            buffer.put_u8((length & 0xff) as u8);
            buffer.put_u8((length >> 8 & 0xff) as u8);
            buffer.put_slice(encrypt_data);
            println!("發送封包至客戶端=========>: {:02X?}", &buffer.to_vec()[..]);
            self.tcp_out.write_all(&buffer.freeze()).await.expect(format!("封包發送到用戶端 ({}) 失敗。", self.addr).as_str());

    }
}