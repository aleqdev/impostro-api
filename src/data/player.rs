use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Player {
    pub name: String,
    pub role: Role,
}

impl Player {
    pub fn new(name: String, role: Role) -> Self {
        Self { name, role }
    }
}
