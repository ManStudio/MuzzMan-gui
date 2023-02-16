use std::io::Write;

#[derive(Clone)]
pub struct Logger {
    pub section_name: String,
    pub stream: std::sync::mpsc::Sender<String>,
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl Logger {
    pub fn new(name: impl Into<String>, channel: std::sync::mpsc::Sender<String>) -> Self {
        Self {
            section_name: name.into(),
            stream: channel,
        }
    }
    pub fn log(&self, msg: impl Into<String>) {
        let _ = self
            .stream
            .send(format!("{}: {}\n", self.section_name, msg.into()));
    }
}

impl Write for Logger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = String::from_utf8_lossy(buf);
        self.log(msg);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
