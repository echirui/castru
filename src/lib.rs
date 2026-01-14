pub mod error;
pub mod proto;
pub mod codec;
pub mod tls;
pub mod client;
pub mod protocol;
pub mod discovery;
pub mod controllers;
pub mod server;
pub mod transcode;

pub use client::CastClient;
pub use error::CastError;
pub use discovery::{CastDevice, discover_devices, discover_devices_async};