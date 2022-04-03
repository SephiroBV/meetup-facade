use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventInfo {
    pub name: String,
    pub venue: String,
    pub attendees: u8,
}
