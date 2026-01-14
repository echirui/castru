use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Heartbeat {
    #[serde(rename = "PING")]
    Ping,
    #[serde(rename = "PONG")]
    Pong,
}

pub const NAMESPACE: &str = "urn:x-cast:com.google.cast.tp.heartbeat";
