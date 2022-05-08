use serde::{Deserialize, Serialize};

use super::{player::Player, resources::Resources};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Group {
    pub name: String,
    pub members: Vec<Player>,
    pub resources: Resources,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            name,
            members: vec![],
            resources: Resources::default(),
        }
    }

    pub fn init_resources(&mut self, rooms_count: usize) {
        self.resources = Resources::new(rooms_count, self.members.iter())
    }
}
