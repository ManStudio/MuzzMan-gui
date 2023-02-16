use iced::Command;

use crate::application::MuzzManInstaller;

#[derive(Debug)]
pub enum Message {
    Command(Command<Message>),
    Close,
    Mimimize,
    Install,
    UnInstall,
    TaskFinished(usize),
    Tick(iced::time::Instant),
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Tick(time) => Message::Tick(*time),
            Message::Close => Self::Close,
            Message::Mimimize => Self::Mimimize,
            Message::Install => Self::Install,
            Message::UnInstall => Self::UnInstall,
            Message::TaskFinished(task) => Self::TaskFinished(task.clone()),
            Message::Command(_) => todo!(),
        }
    }
}

impl MuzzManInstaller {
    pub fn process_logs(&mut self) {
        match self.log_reciver.try_recv() {
            Ok(msg) => self.output_log.push_str(&msg),
            _ => {}
        }
    }
}

impl Message {
    pub fn process(self, app: &mut MuzzManInstaller) -> Command<Message> {
        app.process_logs();
        match self {
            Message::Command(command) => return command,
            Message::Close => return iced::window::close(),
            Message::Mimimize => return iced::window::minimize(true),
            Message::Install => {
                app.installer.arm();
                return app.installer.process();
            }
            Message::UnInstall => {
                todo!("The UnInstallProcess is not implemented");
            }
            Message::Tick(_) => {}
            Message::TaskFinished(task) => {
                println!("Task finished: {task}");
                return app.installer.finished(task);
            }
        }
        Command::none()
    }
}
