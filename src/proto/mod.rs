pub mod extensions {
    pub mod api {
        pub mod cast_channel {
            include!(concat!(env!("OUT_DIR"), "/extensions.api.cast_channel.rs"));
        }
    }
}

// Re-export common types for easier access
pub use extensions::api::cast_channel::*;
