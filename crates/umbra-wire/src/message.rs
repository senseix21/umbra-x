// Message wire format (protobuf generated)

pub mod message {
    include!(concat!(env!("OUT_DIR"), "/umbra.message.rs"));
}

pub use message::{ChatMessage, EncryptedMessage, IdentityAnnouncement, Message};
