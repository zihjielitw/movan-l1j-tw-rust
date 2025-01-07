use crate::utils;

pub struct Cipher {
    // 將亂數數值混淆用的混淆密碼 (32 位元,靜態唯讀) 該數值只有在Cipher初始化時才會被調用
    _1: i32,
    // 初始的解碼數值
    _2: i32,
    // 將亂數數值混淆用的混淆密碼 (32 位元,靜態唯讀) 該數值只有在Cipher初始化時才會被調用
    _3: i32,
    // 將封包數值混淆用的混淆密碼 (32 位元,靜態唯讀) 該數值只有在編碼或解碼時才會被調用
    _4: i32,
    // 參考用的編碼鑰匙 (位元組陣列長度為 8,相當於一個 64 位元的長整數)
    eb: [u8; 8],
    // 參考用的解碼鑰匙 (位元組陣列長度為 8,相當於一個 64 位元的長整數)
    db: [u8; 8],
    // 參考用的封包鑰匙 (位元組陣列長度為 4,相當於一個 32 位元的整數)
    tb: [u8; 4],
}

/*
初始化流程:
1.建立新的鑰匙暫存器(keys),將編碼鑰匙與混淆鑰匙(_1)進行混淆並帶入keys[0],將初始的解碼數值帶入key[1]
2.將key[0]向右反轉19個位元(0x13)並帶入key[0]
3.將key[0]與key[1]與混淆鑰匙(_3)進行混淆並帶入key[1]
4.將keys轉換為64位元的位元組陣列

@param key  由亂數產生的編碼鑰匙
*/
impl Cipher {
    pub fn new(key: i32) -> Cipher {

        let p1: i32 = utils::hex::hex_to_decimal("9c30d539").unwrap_or(0) as i32;
        let p2: i32 = utils::hex::hex_to_decimal("930fd7e2").unwrap_or(0) as i32;
        let p3: i32 = utils::hex::hex_to_decimal("7c72e993").unwrap_or(0) as i32;
        let p4: i32 = utils::hex::hex_to_decimal("287effc3").unwrap_or(0) as i32;

        let mut eb_new = [0u8; 8];
        let mut db_new = [0u8; 8];
        let tb_new = [0u8; 4];

        let mut keys = [key ^ p1, p2];
        keys[0] = (((keys[0] << 0x13) as u32) | ((keys[0] as u32) >> 13)) as i32;
        keys[1] ^= keys[0] ^ p3;

        for i in 0..keys.len() {
            for j in 0..tb_new.len() {
                eb_new[(i*4)+j] =  (keys[i] >> (j * 8) & 0xff)  as u8
            }
        }

        db_new.clone_from_slice(&eb_new);

        Cipher {
            _1: p1,
            _2: p2,
            _3: p3,
            _4: p4,
            eb: eb_new,
            db: db_new,
            tb: tb_new,
        }
    }

    /**
    * 將未受保護的資料進行混淆,避免資料外流.
    *
    * @param data, 未受保護的資料
    * @return data, 受保護的資料
    */
    pub fn encrypt<'a>(&mut self, data: &'a mut [u8]) -> &'a [u8] {

        for i in 0..self.tb.len() {
            self.tb[i] = data[i];
        }


        data[0] ^= self.eb[0];

        for i in 1..data.len() {
            data[i] ^= data[i-1] ^ self.eb[i&7]
        }

        data[3] ^= self.eb[2];
        data[2] ^= self.eb[3] ^ data[3];
        data[1] ^= self.eb[4] ^ data[2];
        data[0] ^= self.eb[5] ^ data[1];

        self.update_eb();
       
        data
    }

    /**
     * 將受保護的資料進行還原，讓伺服器可以參考正確的資料.
     *
     * @param data		加密資料
     * @return data		原始資料
     **/
    pub fn decrypt<'a>(&mut self, data: &'a mut [u8]) ->  &'a [u8] {
        data[0] ^= self.db[5] ^ data[1];
        data[1] ^= self.db[4] ^ data[2];
        data[2] ^= self.db[3] ^ data[3];
        data[3] ^= self.db[2];

        println!("data: {:02X?}", &data);
        println!("db: {:02X?}", &self.db);


        for i in (1..data.len()).rev() {
            data[i] ^= data[i-1] ^ self.db[i&7]
        }

        data[0] ^= self.db[0];
        println!("解密: {:02X?}", &data);
        self.update_db(data.to_vec());
        data
    }

    /**
    * 將指定的鑰匙進行混淆並與混淆鑰匙相加 (_4)
    *
    * @param ref		原始資料
    */
    pub fn update_db(&mut self, ref_data: Vec<u8>) {


        for i in 0..self.tb.len() {
            self.db[i] ^= ref_data[i]
        }

        let v_i32 = (i32::from(self.db[7]&0xFF) << 24 | i32::from(self.db[6]&0xFF) << 16 | i32::from(self.db[5]&0xFF) << 8 | i32::from(self.db[4]&0xFF)) + self._4;
/*
        println!("self.db[7]&0xFF) << 24: {}", i32::from(self.db[7] & 0xFF) << 24);
        println!("self.db[6]&0xFF) << 16: {}", i32::from(self.db[6] & 0xFF) << 16);
        println!("self.db[5]&0xFF) << 8: {}", i32::from(self.db[5] & 0xFF) << 8);
        println!("self.db[4]&0xFF): {}", i32::from(self.db[4] & 0xFF) );
        println!("_4: {}",  self._4);
        println!("v_i32: {}", v_i32);*/

        for i in 0..self.tb.len() {
            self.db[i+4] = (v_i32 >> (i * 8) & 0xff) as u8;
        }
    }

    /**
    * 將指定的鑰匙進行混淆並與混淆鑰匙相加 (_4)
    *
    * @param ref		原始資料
    */
    pub fn update_eb(&mut self) {
        for i in 0..self.tb.len() {
            self.eb[i] ^= self.tb[i]
        }

        let v_i32 = (i32::from(self.eb[7]&0xFF) << 24 | i32::from(self.eb[6]&0xFF) << 16 | i32::from(self.eb[5]&0xFF) << 8 | i32::from(self.eb[4]&0xFF)) + self._4;
        for i in 0..self.tb.len() {
            self.eb[i+4] = (v_i32 >> (i * 8) & 0xff) as u8;
        }
    }
}