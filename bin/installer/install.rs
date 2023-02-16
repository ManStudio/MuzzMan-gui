use std::{future::Future, pin::Pin};

use async_trait::async_trait;

use crate::{logger::Logger, logic::Message};

pub type Channel = std::sync::mpsc::Sender<String>;

pub struct Installer {
    pub steps: Vec<(
        Box<dyn Fn(Channel) -> Pin<Box<dyn Future<Output = ()> + Send>>>,
        Vec<usize>,
    )>,
    pub to_do: Vec<(usize, bool)>,
    pub channel: std::sync::mpsc::Sender<String>,
}

impl Installer {
    pub fn new(log_sender: Channel) -> Self {
        Self {
            channel: log_sender,
            steps: Vec::new(),
            to_do: Vec::new(),
        }
    }
    pub fn add_step<T: 'static + Fn(Channel) -> Pin<Box<dyn Future<Output = ()> + Send>>>(
        &mut self,
        step: T,
        depends_on: impl Into<Vec<usize>>,
    ) -> usize {
        let id = self.steps.len();
        self.steps.push((Box::new(step), depends_on.into()));
        id
    }

    pub fn arm(&mut self) {
        self.to_do.clear();
        self.to_do
            .append(&mut (0..self.steps.len()).map(|e| (e, false)).collect())
    }

    pub fn finished(&mut self, id: usize) -> iced::Command<Message> {
        self.to_do.retain(|e| e.0 != id);
        self.process()
    }

    pub fn process(&mut self) -> iced::Command<Message> {
        let mut commands = Vec::new();
        let mut need_to_start_work = Vec::new();

        'step: for (i, (id, working)) in self.to_do.iter().enumerate() {
            if !working {
                // the current step need to have no dependences in self.to_do
                for dependent in &self.steps[*id].1 {
                    if self.to_do.contains(&(*dependent, false))
                        || self.to_do.contains(&(*dependent, true))
                    {
                        continue 'step;
                    }
                }

                need_to_start_work.push(i)
            }
        }

        for i in need_to_start_work {
            let to_do = &mut self.to_do[i];
            to_do.1 = true;
            let id = to_do.0;
            let step = (self.steps[to_do.0].0)(self.channel.clone());
            commands.push(iced::Command::perform(step, move |_| {
                Message::TaskFinished(id)
            }));
        }

        iced::Command::batch(commands)
    }
}
