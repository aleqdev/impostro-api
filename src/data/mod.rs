use serde::{Deserialize, Serialize};

use self::session::{Session, SessionCounter, SessionId};

pub mod group;
pub mod player;
pub mod resources;
pub mod role;
pub mod session;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize)]
pub struct ImpostroData {
    pub sessions: Vec<Session>,
    pub session_counter: SessionCounter,
}

impl ImpostroData {
    pub fn get_session<'a, 'b: 'a>(&'b self, id: SessionId) -> Option<&'a Session> {
        for s in &self.sessions {
            if s.id == id {
                return Some(s);
            }
        }
        return None;
    }
}
