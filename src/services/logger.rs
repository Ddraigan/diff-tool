use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use log::LevelFilter;

pub struct VecWriter {
    logs: Arc<Mutex<Vec<String>>>,
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

impl VecWriter {
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self { logs }
    }
}

pub fn init_logging(writer: VecWriter, level: LevelFilter) {
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(writer)))
        .filter(None, level)
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .init()
}
