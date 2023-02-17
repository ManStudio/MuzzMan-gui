use std::io::{BufRead, BufReader, Read};

use iced::Command;

use crate::{application::MuzzManInstaller, logger::Logger, task_manager::TaskManager};

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
        loop {
            match self.log_reciver.try_recv() {
                Ok(msg) => self.output_log.push_str(&msg),
                _ => break,
            }
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
                install_tasks(&mut app.installer);
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

pub fn install_tasks(manager: &mut TaskManager) {
    manager.clear();

    let rust_up = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("RustUp", channel);
                loop {
                    if std::process::Command::new("rustup").output().is_ok() {
                        logger.log("RustUp is installed!");
                        return;
                    } else {
                        logger.log("You should install rustup");
                        logger.log("You can install rustup from https://rustup.rs/");
                        logger.log("RustUp Installed finished");
                        std::thread::sleep(std::time::Duration::from_secs(5))
                    }
                }
            })
        },
        vec![],
    );

    let update_rust = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("Rust Update", channel);
                let mut process = std::process::Command::new("rustup")
                    .arg("update")
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
                    .expect("rustup is not installed!");
                let stdout = process.stdout.take().unwrap();
                let stderr = process.stderr.take().unwrap();
                let stdout_reader = BufReader::new(stdout);
                let stderr_reader = BufReader::new(stderr);

                let logger_1 = logger.clone();
                let logger_2 = logger.clone();

                let t1 = std::thread::spawn(move || {
                    stdout_reader.lines().for_each(|line| {
                        if let Ok(line) = line {
                            logger_1.log(line)
                        }
                    })
                });

                let t2 = std::thread::spawn(move || {
                    stderr_reader.lines().for_each(|line| {
                        if let Ok(line) = line {
                            logger_2.log(line)
                        }
                    })
                });

                let _ = t1.join();
                let _ = t2.join();
                logger.log("Finished!");
            })
        },
        vec![rust_up],
    );

    let install_stable = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("Rust stable toolchain", channel);
                logger.log("Install Stable");
            })
        },
        vec![rust_up],
    );
}
