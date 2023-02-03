use muzzman_daemon::prelude::LocationId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleConfig {
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
