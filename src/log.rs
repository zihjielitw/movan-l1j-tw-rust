use rolling_file::{BasicRollingFileAppender, RollingConditionBasic};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use time::{macros::format_description, UtcOffset};

pub struct ServerLog {}

impl ServerLog {

    pub fn init_log() -> Vec<tracing_appender::non_blocking::WorkerGuard> {
        let mut guards = vec![];

        let full_file = BasicRollingFileAppender::new(
            "./logs/server.log",
            RollingConditionBasic::new().hourly(),
            24
        ).unwrap();
        //let full_file = rolling::hourly("./logs", "game_server.log");
        let (full_appender, full_appender_guard) = tracing_appender::non_blocking(full_file);

        let error_file = BasicRollingFileAppender::new(
            "./logs/error.log",
            RollingConditionBasic::new().daily(),
            3
        ).unwrap();
       // let error_file = rolling::daily("./logs", "error.log");
        let (error_appender, error_appender_guard) = tracing_appender::non_blocking(error_file);

        let (stdout_appender, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());

        guards.push(full_appender_guard);
        guards.push(error_appender_guard);
        guards.push(stdout_guard);

        let mk_writer = full_appender.with_max_level(tracing::Level::INFO)
            //.and(console_appender.with_min_level(tracing::Level::INFO).with_max_level(tracing::Level::INFO))
            .and(error_appender.with_max_level(tracing::Level::ERROR))
            .and(stdout_appender.with_max_level(tracing::Level::TRACE));

        let local_time = OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
        );

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_writer(mk_writer)
            .with_ansi(false)
            .with_timer(local_time)
            .init();

        guards
    }
}