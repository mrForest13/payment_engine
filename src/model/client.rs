use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ClientId(pub u16);

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
