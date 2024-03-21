use std::{
    io::Write,
    sync::{Arc, Mutex},
};

pub struct VecWriter {
    pub logs: Arc<Mutex<Vec<String>>>,
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
