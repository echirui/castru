use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Connection {
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "CLOSE")]
    Close,
}

pub const NAMESPACE: &str = "urn:x-cast:com.google.cast.tp.connection";
