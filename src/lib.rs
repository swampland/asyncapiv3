pub mod spec;
#[cfg(feature = "builder_unstable")]
pub mod builder;
#[cfg(feature = "builder_unstable")]
pub mod error;

// Re-export asyncapi bindings for direct access
pub mod bindings {
    pub use asyncapi::{
        ServerBinding,
        ChannelBinding,
        MessageBinding,
        OperationBinding,
        server_binding::*,
        channel_binding::*,
        message_binding::*,
        operation_binding::*,
    };
}