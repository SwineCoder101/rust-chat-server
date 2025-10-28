// messages.rs (or an inline `mod messages { ... }`)
// Make types & functions public.
use iroh::EndpointId;
use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub body: MessageBody,
    pub nonce: [u8; 16],
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageBody {
    AboutMe { from: EndpointId, name: String },
    Message { from: EndpointId, text: String },
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn new(body: MessageBody) -> Self {
        Self { body, nonce: rand::random() }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}
