use std::{
    fs::File,
    io::{Read, Seek, Write},
    ops::{Deref, DerefMut},
    path::Path,
};

use muzzman_daemon::prelude::LocationId;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "tick_default")]
    pub tick: u64,
    #[serde(default = "location_default")]
    pub location_id: LocationId,
    #[serde(default = "destroy_element_default")]
    pub destroy_element: bool,
}

fn tick_default() -> u64 {
    100
}

fn location_default() -> LocationId {
    LocationId(Vec::new())
}

fn destroy_element_default() -> bool {
    true
}

// Wrap

pub struct WrapConfig<T: Serialize + DeserializeOwned> {
    pub data: T,
    pub file: File,
}

impl<T: Serialize + DeserializeOwned> WrapConfig<T> {
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let data = toml::from_str::<T>(&data).unwrap();
        Ok(Self { data, file })
    }

    pub fn reload(&mut self) {
        let mut data = String::new();
        self.file.rewind().unwrap();
        self.file.read_to_string(&mut data).unwrap();
        let new = toml::from_str::<T>(&data).unwrap();
        self.data = new;
    }

    pub fn update(&mut self) {
        let data = toml::to_string(&self.data).unwrap();
        self.file.rewind().unwrap();
        self.file.write_all(data.as_bytes()).unwrap()
    }
}

// here was implemented Drop for WrapConfig!!!
// If the settings is closed but the simple, manager or progress is open then is closed the
// settings will be override

impl<T: Serialize + DeserializeOwned> Deref for WrapConfig<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Serialize + DeserializeOwned> DerefMut for WrapConfig<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
