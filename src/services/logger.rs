use super::config::get_data_dir;
use anyhow::Result;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};
use tracing::{Event, Subscriber};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    self,
    fmt::MakeWriter,
    layer::{Context, SubscriberExt},
    util::SubscriberInitExt,
    Layer,
};

lazy_static::lazy_static! {
  pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
  pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
  pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

pub type Logs = Arc<Mutex<Vec<String>>>;

#[derive(Clone)]
pub struct VecWriter {
    logs: Logs,
}

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = String::from_utf8_lossy(buf);
        let mut logs = self.logs.lock().unwrap();

        logs.push(msg.into_owned());

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'writer> MakeWriter<'writer> for VecWriter {
    type Writer = VecWriter;

    fn make_writer(&'writer self) -> Self::Writer {
        self.clone()
    }
}

impl<S> Layer<S> for VecWriter
where
    S: Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // Log to custom destination (e.g., Vec)
        self.logs
            .lock()
            .unwrap()
            .push(format!("{}", event.metadata().name()));
    }
}

impl VecWriter {
    pub fn new(logs: Logs) -> Self {
        Self { logs }
    }
}

pub fn initialize_logging(vec_writer: VecWriter) -> Result<()> {
    let directory = get_data_dir()?;
    std::fs::create_dir_all(directory.clone())?;
    let log_path = directory.join(LOG_FILE.clone());
    let file_writer = std::fs::File::create(log_path)?;

    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG")
            .or_else(|_| std::env::var(LOG_ENV.clone()))
            .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
    );

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_writer)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    let console_suscriber = tracing_subscriber::fmt::layer()
        .with_file(false)
        .with_line_number(false)
        .with_writer(vec_writer)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .with(console_suscriber)
        .init();
    Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr, $ex:expr) => {
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}

//
// pub fn init_logging(writer: VecWriter, level: LevelFilter) {
//     // let data_dir = get_data_dir().unwrap();
//     // std::fs::create_dir_all(data_dir.clone()).unwrap();
//     // let log_path = data_dir.join(LOG_FILE.clone());
//     // let log_file = std::fs::File::create(log_path).unwrap();
//
//     env_logger::Builder::new()
//         .target(env_logger::Target::Pipe(Box::new(writer)))
//         .filter(None, level)
//         .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
//         .init()
// }
