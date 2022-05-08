use rand::{self, Rng};
use serde::{Deserialize, Serialize};

use super::group::Group;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize)]
pub struct SessionCounter(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SessionId(pub [char; 6]);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Session {
    pub id: SessionId,
    pub groups: Vec<Group>,
}

impl SessionId {
    pub fn new_random(SessionCounter(ref mut counter): &mut SessionCounter) -> Self {
        let mut rng = rand::thread_rng();
        let mut val: [char; 6] = ['0'; 6];
        for x in val[1..].iter_mut() {
            *x = rng.gen_range(b'A'..=b'Z') as char;
        }
        val[0] = *counter.to_string().as_bytes().last().unwrap() as char;
        *counter += 1;
        Self(val)
    }
}
