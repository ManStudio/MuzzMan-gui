use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, Write},
    path::PathBuf,
};

use iced::Command;
#[cfg(target_os = "windows")]
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
    ChangeLocal(bool),
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
            Message::ChangeLocal(value) => Self::ChangeLocal(*value),
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
                app.install_tasks();
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
            Message::ChangeLocal(value) => app.local = value,
        }
        Command::batch(commands)
    }
}

impl MuzzManInstaller {
    pub fn install_tasks(&mut self) {
        let manager = &mut self.manager;
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

        let git = manager.add_step(
            |channel| {
                Box::pin(async {
                    let logger = Logger::new("Git", channel);

                    loop {
                        if std::process::Command::new("git").output().is_ok() {
                            logger.log("Is installed!");
                            return;
                        } else {
                            logger.log("You should install git");
                            logger.log("You can install git from https://git-scm.com/");
                            logger.log("Installed finished");
                            std::thread::sleep(std::time::Duration::from_secs(5))
                        }
                    }
                })
            },
            vec![],
        );

        let git_submodule_update = if self.local {
            manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("Git Submodule Update", channel);

                        execute_command(
                            std::process::Command::new("git")
                                .arg("submodule")
                                .arg("update")
                                .arg("--recursive")
                                .arg("--init")
                                .arg("--remote"),
                            &logger,
                        );

                        logger.log("Finished");
                    })
                },
                vec![git],
            )
        } else {
            panic!("Not implemented!")
        };

        let update_rust = manager.add_step(
            |channel| {
                Box::pin(async {
                    let logger = Logger::new("Rust Update", channel);
                    execute_command(std::process::Command::new("rustup").arg("update"), &logger);
                    logger.log("Finished!");
                })
            },
            vec![rust_up, git],
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

        let daemon_build = if self.local {
            manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("Build Daemon", channel);
                        std::env::set_current_dir(PathBuf::from("muzzman-daemon"))
                            .expect("Expecting muzzman-daemon repo");
                        execute_command(
                            std::process::Command::new("cargo")
                                .arg("build")
                                .arg("--release"),
                            &logger,
                        );
                        std::env::set_current_dir(PathBuf::from("..")).unwrap();
                        logger.log("Builded!");
                    })
                },
                vec![git_submodule_update, install_stable],
            )
        } else {
            todo!()
        };

        let build = if self.local {
            manager.add_step(
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
            )
        } else {
            panic!("Online install not implemented!")
        };

        let local_bin = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("Local bin", channel);
                logger.log("Verify if local bin folder exist");
                logger.log(
                    "Will create a ~/.local/bin folder or %AppData%\\Local\\MuzzMan\\bin if not exist",
                );

                let bin_path = get_bin_path();

                let Some(bin_path) = bin_path else{logger.log("You are on a invalid os!");return};

                if !bin_path.is_dir(){
                    let _ = std::fs::create_dir(bin_path);
                }
            })
        },
        vec![rust_up],);

        let setup_path = manager.add_step(
        |channel| {
            Box::pin(async {
                let logger = Logger::new("setup_path", channel);
                let bin_path = get_bin_path().expect("Cannot get local bin");
                logger.log(format!("Bin path: {}", bin_path.to_str().unwrap()));

                let path = std::env::var("PATH").unwrap_or_else(|_| { panic!("{}", bin_path.to_str().unwrap().to_string()) });
                if !path.contains(bin_path.to_str().unwrap()) {
                    #[cfg(target_os = "linux")]
                    std::env::set_var("PATH", format!("{}:{}", bin_path.to_str().unwrap(), path));
                    #[cfg(target_os = "windows")]
                    std::env::set_var("PATH", format!("{};{}", bin_path.to_str().unwrap(), path));
                    logger.log("Added to local path!");

                    #[cfg(target_os = "linux")]
                    {
                        if let Ok(shell) = std::env::var("SHELL"){
                            if shell.contains("bash"){
                                let mut file = File::options().write(true).open(format!("{}/.bashrc", dirs::home_dir().unwrap().to_str().unwrap())).unwrap();
                                file.seek(std::io::SeekFrom::End(0)).unwrap();
                                file.write_all(format!("export PATH=\"{}:$PATH\"\n", bin_path.to_str().unwrap()).as_bytes()).unwrap();
                            }else if shell.contains("zsh"){
                                let mut file = File::options().write(true).open(format!("{}/.zshrc", dirs::home_dir().unwrap().to_str().unwrap())).unwrap();
                                file.seek(std::io::SeekFrom::End(0)).unwrap();
                                file.write_all(format!("export PATH=\"{}:$PATH\"\n", bin_path.to_str().unwrap()).as_bytes()).unwrap();
                            }else{
                                logger.log("Error: Path cannot be updated you need to add ~/.local/bin to you path")
                            }

                        }else{
                            logger.log("Error: Cannot find that means will not be in the path you need to add ~/.local/bin to your path");
                            return
                        }
                    }
                    #[cfg(target_os = "windows")]
                    {
                        use winreg::{enums::*, RegKey};
                        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                        let (env, _) = hkcu.create_subkey("Environment").unwrap();
                        env.set_value("PATH", &std::env::var("PATH").unwrap());
                    }
                    logger.log("Finished PATH");
                }
            })
        },
        vec![rust_up],);

        if self.local {
            fn install(name: &str, from: PathBuf) -> std::io::Result<u64> {
                let path = get_bin_path().unwrap();

                std::fs::copy(
                    from.join("target")
                        .join("release")
                        .join(format!("{name}{}", std::env::consts::EXE_EXTENSION)),
                    path.join(format!("{name}{}", std::env::consts::EXE_EXTENSION)),
                )
            }

            let install_muzzman_daemon = manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("MuzzMan Daemon", channel);
                        // stop systemd service if exists
                        if let Ok(mut process) = std::process::Command::new("systemctl")
                            .arg("--user")
                            .arg("stop")
                            .arg("muzzman-daemon.service")
                            .spawn()
                        {
                            let _ = process.wait();
                        }
                        logger.log("Installing");
                        install("muzzman-daemon", PathBuf::from("muzzman-daemon")).unwrap();
                        logger.log("Installed");

                        logger.log("Create Service!");
                        #[cfg(target_os = "linux")]
                        {
                            let mut systemd =
                                dirs::home_dir().unwrap().join(".config").join("systemd");
                            // has systemd
                            if systemd.is_dir() {
                                systemd = systemd.join("user");
                                let _ = std::fs::create_dir_all(&systemd);
                                let service_path = systemd.join("muzzman-daemon.service");

                                let mut file = std::fs::File::options()
                                    .write(true)
                                    .create(true)
                                    .open(service_path)
                                    .unwrap();
                                let _ = file.rewind();
                                let bin_path = get_bin_path().unwrap();
                                write!(
                                    file,
                                    "[Unit]
Description = MuzzMan Daemon

[Service]
ExecStart = {}/muzzman-daemon

[Install]
WantedBy = default.target",
                                    bin_path.to_str().unwrap()
                                )
                                .unwrap();
                                logger.log("Create Systemd user service!");
                                std::process::Command::new("systemctl")
                                    .arg("--user")
                                    .arg("enable")
                                    .arg("muzzman-daemon.service")
                                    .arg("--now")
                                    .spawn()
                                    .unwrap()
                                    .wait()
                                    .unwrap();
                                logger.log("Systemd service enabled and started!");
                            }
                        }
                        #[cfg(target_os = "windows")]
                        {}
                        logger.log("Service created!");
                    })
                },
                vec![daemon_build],
            );

            let install_muzzman_simple = manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("MuzzMan Simple", channel);
                        logger.log("Installing");

                        install("muzzman_simple", PathBuf::from(".")).unwrap();

                        logger.log("Installed!");
                    })
                },
                vec![build],
            );

            let install_muzzman_simple_settings = manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("MuzzMan Simple Settings", channel);
                        logger.log("Installing");

                        install("muzzman_simple_settings", PathBuf::from(".")).unwrap();

                        logger.log("Installed!");
                    })
                },
                vec![build],
            );

            let install_muzzman_progress = manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("MuzzMan Progress", channel);
                        logger.log("Installing");

                        install("muzzman_progress", PathBuf::from(".")).unwrap();

                        logger.log("Installed!");
                    })
                },
                vec![build],
            );

            let install_muzzman_manager = manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("MuzzMan Manager", channel);
                        logger.log("Installing");

                        install("muzzman_manager", PathBuf::from(".")).unwrap();

                        logger.log("Installed!");
                    })
                },
                vec![build],
            );

            let install_muzzman_settings = manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("MuzzMan Settings", channel);
                        logger.log("Installing");

                        install("muzzman_settings", PathBuf::new()).unwrap();

                        logger.log("Installed!");
                    })
                },
                vec![build],
            );

            let _install = manager.add_step(
                |channel| {
                    Box::pin(async {
                        let logger = Logger::new("MuzzMan Installer", channel);
                        logger.log("Install succesful!");
                    })
                },
                vec![
                    install_muzzman_simple,
                    install_muzzman_simple_settings,
                    install_muzzman_progress,
                    install_muzzman_manager,
                    install_muzzman_settings,
                ],
            );
        } else {
            panic!("Online install not implemented!")
        }
    }
}

pub fn get_bin_path() -> Option<PathBuf> {
    let bin_path;

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
