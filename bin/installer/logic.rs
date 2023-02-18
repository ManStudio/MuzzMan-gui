use std::{
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use iced::Command;
use muzzman_daemon::common::get_muzzman_dir;

use crate::{application::MuzzManInstaller, logger::Logger, task_manager::TaskManager};

#[derive(Debug)]
pub enum Message {
    Command(Command<Message>),
    Close,
    Mimimize,
    Install,
    UnInstall,
    TaskFinished(usize),
    ChangeAutoScroll(bool),
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
            Message::ChangeAutoScroll(value) => Self::ChangeAutoScroll(*value),
            Message::Command(_) => todo!(),
        }
    }
}

impl MuzzManInstaller {
    pub fn process_logs(&mut self) {
        let mut tmsg = self.log_reciver.try_recv();
        while tmsg.is_ok() {
            if let Ok(mut msg) = tmsg {
                tmsg = self.log_reciver.try_recv();
                msg.1.push('\n');
                let msgs = msg.1.split(['\n', '\r']).collect::<Vec<&str>>();
                let section = &msg.0;
                for msg in msgs {
                    if msg.is_empty() {
                        continue;
                    }
                    self.section_log.push_str(section);
                    self.section_log.push('\n');

                    self.output_log.push_str(msg);
                    self.output_log.push('\n');
                }
            }
        }
    }
}

impl Message {
    pub fn process(self, app: &mut MuzzManInstaller) -> Command<Message> {
        let mut commands = Vec::new();
        app.process_logs();
        if app.auto_scroll {
            commands.push(iced::widget::scrollable::snap_to(
                app.output_scroll_id.clone(),
                iced::widget::scrollable::RelativeOffset { y: 1.0, x: 0.0 },
            ));
        }
        match self {
            Message::Command(command) => commands.push(command),
            Message::Close => {
                if app.manager.to_do.is_empty() {
                    commands.push(iced::window::close());
                } else {
                    app.should_close = true;
                }
            }
            Message::Mimimize => commands.push(iced::window::minimize(true)),
            Message::Install => {
                install_tasks(&mut app.manager);
                app.manager.arm();
                commands.push(app.manager.process());
            }
            Message::UnInstall => {
                todo!("The UnInstallProcess is not implemented");
            }
            Message::Tick(_) => {}
            Message::TaskFinished(task) => {
                println!("Task finished: {task}");
                commands.push(app.manager.finished(task));
                if app.should_close && app.manager.to_do.is_empty() {
                    commands.push(iced::window::close());
                }
            }
            Message::ChangeAutoScroll(value) => app.auto_scroll = value,
        }
        Command::batch(commands)
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
                execute_command(std::process::Command::new("rustup").arg("update"), &logger);
                logger.log("Finished!");
            })
        },
        vec![rust_up],
    );

    let install_stable = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("Rust stable toolchain", channel);
                execute_command(
                    std::process::Command::new("rustup")
                        .arg("install")
                        .arg("stable"),
                    &logger,
                );
                logger.log("Finished");
            })
        },
        vec![update_rust],
    );

    let build = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("build", channel);
                execute_command(
                    std::process::Command::new("cargo")
                        .arg("build")
                        .arg("--release"),
                    &logger,
                );
                logger.log("Builded!");
            })
        },
        vec![install_stable],
    );

    let local_bin = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("Local bin", channel);
                logger.log("Verify if local bin folder exist");
                logger.log(
                    "Will create a ~/.local/bin folder or %AppData%\\Local\\MuzzMan\\bin if not exist",
                );

                let bin_path = get_bit_path();

                let Some(bin_path) = bin_path else{logger.log("You are on a invalid os!");return};

                if !bin_path.is_dir(){
                    let _ = std::fs::create_dir(bin_path);
                }
            })
        },
        vec![rust_up],
    );

    let setup_path = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("setup_path", channel);
                let bin_path = get_bit_path().expect("Cannot get local bin");
                logger.log("Finished PATH");
                logger.log(format!("Bin path: {}", bin_path.to_str().unwrap()));
                println!("{:?}", std::env::var("PATH"));

                let path = std::env::var("PATH").expect("Path");
                if !path.contains(bin_path.to_str().unwrap()) {
                    logger.log("Added to path");
                    #[cfg(target_os = "linux")]
                    std::env::set_var("PATH", format!("{}:{}", bin_path.to_str().unwrap(), path));
                    #[cfg(target_os = "windows")]
                    std::env::set_var("PATH", format!("{};{}", bin_path.to_str().unwrap(), path));

                    #[cfg(target_os = "windows")]
                    {
                        use winreg::{enums::*, RegKey};
                        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                        let (env, _) = hkcu.create_subkey("Environment").unwrap();
                        env.set_value("PATH", &std::env::var("PATH").unwrap());
                    }
                }
            })
        },
        vec![rust_up],
    );

    let install = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("install", channel);
                logger.log("Installed!");
            })
        },
        vec![build],
    );
}

pub fn get_bit_path() -> Option<PathBuf> {
    let mut bin_path = None;

    #[cfg(target_os = "windows")]
    {
        bin_path = Some(get_muzzman_dir().join("bin"))
    }
    #[cfg(target_os = "linux")]
    {
        bin_path = Some(
            dirs::home_dir()
                .expect("No home dir!")
                .join(".local")
                .join("bin"),
        );
    }

    bin_path
}

pub fn execute_command(command: &mut std::process::Command, logger: &Logger) {
    let mut process = command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Invalid Command!");

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
}
