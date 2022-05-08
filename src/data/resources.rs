use serde::{Deserialize, Serialize};

use super::{player::Player, role::Role};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize)]
pub struct Resources {
    pub food: usize,
    pub water: usize,
    pub air: usize,
    pub will: usize,
}

impl Resources {
    pub fn new<'a>(rooms_count: usize, members: impl Iterator<Item = &'a Player>) -> Self {
        let mut ordinary = 0;
        let mut will = 0;
        for member in members {
            match member.role {
                Role::Captain => {
                    ordinary += 2;
                    will += 10
                }
                Role::Impostor => ordinary += 3,
                _ => ordinary += 2,
            }
        }
        ordinary *= rooms_count;
        Self {
            food: ordinary,
            water: ordinary,
            air: ordinary,
            will,
        }
    }
}
