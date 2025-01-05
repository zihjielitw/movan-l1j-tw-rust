#[allow(dead_code)]
pub enum Opcodes {
    // 請求登錄伺服器【beanfun】
    COpcodeBeanfunLoginPacket,
    // 請求切換角色
    COpcodeChangeChar,
    // 請求驗證客戶端版本
    COpcodeClientVersion,
    // 請求更新連線狀態
    COpcodeKeepAlive,
    // 請求登錄角色
    COpcodeLoginToServer,
    // 請求配置角色設定
    COpcodeLoginToServerOk,
    // 初始化演算法
    SOpcodeInitPacket,
    // 伺服器版本
    SOpcodeServerVersion,
}

impl Opcodes {
    pub fn value(&self) -> i32 {
        match *self {
            Opcodes::COpcodeChangeChar => 7,
            Opcodes::COpcodeClientVersion => 14,
            Opcodes::COpcodeLoginToServerOk => 26,
            Opcodes::COpcodeKeepAlive => 95,
            Opcodes::COpcodeLoginToServer => 137,
            Opcodes::SOpcodeServerVersion => 139,
            Opcodes::COpcodeBeanfunLoginPacket => 210,
            Opcodes::SOpcodeInitPacket => 150,
        }
    }
}