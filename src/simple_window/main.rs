use flags::Flags;
use iced::{Application, Command};
use logic::Message;
use muzzman_daemon::{
    common::get_muzzman_dir,
    prelude::{ERef, LocationId, TElement, TLocation, TSession},
    DaemonSession,
};
use muzzman_iced::{config::WrapConfig, simple_config::SimpleConfig, themes::get_theme};

mod flags;
mod get_filename;
mod logic;
pub mod render;

pub struct MuzzManSimple {
    url: String,
    downloading: bool,
    status: String,
    progress: f32,
    element: Option<ERef>,
    session: Box<dyn TSession>,
    config: WrapConfig<SimpleConfig>,
}

impl Application for MuzzManSimple {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut tmp_url = String::new();
        let mut tmp_element = None;
        let mut tmp_status = String::from("No element! Put the url and press download!");
        let session = DaemonSession::new().expect("Cannot bind on a port!");
        let session = session.create_session();
        let _ = session
            .get_default_location()
            .expect("Daemon is not started!");

        let iced_dir = get_muzzman_dir().join("iced");
        if !iced_dir.is_dir() {
            std::fs::create_dir_all(iced_dir).unwrap();
        }

        let config_path = if let Some(config_path) = flags.config {
            config_path
        } else {
            get_muzzman_dir().join("iced").join("simple_config.toml")
        };

        let config = WrapConfig::<SimpleConfig>::load(&config_path).unwrap();

        if let Some(command) = flags.command {
            match command {
                flags::Command::Download {
                    url,
                    location_id,
                    name,
                    download,
                } => {
                    tmp_url = url;
                    let name = if let Some(name) = name {
                        name
                    } else {
                        tmp_url.split('/').last().expect("Invalid url").to_owned()
                    };
                    let location = if let Some(location_id) = location_id {
                        let id: LocationId =
                            serde_json::from_str(&location_id).expect("Invalid location id!");
                        session.get_location_ref(&id).unwrap()
                    } else {
                        session.get_default_location().unwrap()
                    };
                    let element = location.create_element(&name).unwrap();
                    element.set_url(Some(tmp_url.clone())).unwrap();
                    if download {
                        let res = element.resolv_module().unwrap();
                        tmp_status = if res {
                            "Resolved!".to_string()
                        } else {
                            "Cannot resolv! You don't have the module or url is invalid!"
                                .to_string()
                        };
                        element.set_enabled(true, None).unwrap();
                    }
                    tmp_element = Some(element);
                }
                flags::Command::Attach { element_id } => {
                    let id = serde_json::from_str(&element_id).expect("Invalid element id!");
                    let element = session
                        .get_element_ref(&id)
                        .expect("Is posibile that the element get destroyed or is a invalid id");
                    tmp_element = Some(element)
                }
            }
        }

        let res = Self {
            url: tmp_url,
            downloading: false,
            status: tmp_status,
            element: tmp_element,
            session,
            config,
            progress: 0.0,
        };

        (res, Command::none())
    }

    fn title(&self) -> String {
        String::from("MuzzMan Simple")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        message.progess(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.render()
    }

    fn theme(&self) -> Self::Theme {
        get_theme()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let mut res = vec![iced::subscription::events().map(Message::Event)];

        res.push(
            iced::time::every(std::time::Duration::from_millis(self.config.tick))
                .map(Message::Tick),
        );

        iced::Subscription::batch(res)
    }
}

impl Drop for MuzzManSimple {
    fn drop(&mut self) {
        if self.config.destroy_element {
            if let Some(element) = self.element.take() {
                let _ = element.destroy();
            }
        }
    }
}

fn main() {
    env_logger::init();
    MuzzManSimple::run(iced::Settings {
        window: iced::window::Settings {
            decorations: false,
            resizable: false,
            size: (500, 95),
            max_size: Some((500, 95)),
            min_size: Some((500, 95)),
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}
