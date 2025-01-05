use tokio::sync::OnceCell;


static GLOBAL_GAME_CONTEXT: OnceCell<GameContext> = OnceCell::const_new();

pub struct GameContext {
    pub game_server_start_timestamp: i32,
    pub game_server_start_time: String,
}

impl GameContext {
    pub async fn init() {
        GLOBAL_GAME_CONTEXT.get_or_init(|| async {
            Self::new()
        }).await;
    }

    pub fn get_context() -> Option<&'static GameContext> {
        GLOBAL_GAME_CONTEXT.get()
    }


    fn new() -> Self {

        let now = chrono::offset::Local::now();

        let context_data: GameContext = GameContext {
            game_server_start_timestamp: (now.timestamp_millis() / 1000) as i32,
            game_server_start_time: now.to_string(),
        };

        context_data
    }
}