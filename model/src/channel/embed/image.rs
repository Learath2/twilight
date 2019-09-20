use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmbedImage {
    pub height: u64,
    pub proxy_url: String,
    pub url: String,
    pub width: u64,
}