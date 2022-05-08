use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Role {
    Captain,
    Medic,
    Engineer,
    Pilot,
    Impostor,
}
