use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct OutboundData {
    #[serde(flatten)]
    inner: HashMap<String, String>,
}

impl OutboundData {
    pub fn page<P: Into<String>, C: Into<String>>(
        mut self,
        property: P,
        content: C,
    ) -> OutboundData {
        self.inner.insert(property.into(), content.into());
        self
    }

    pub fn build(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
    }
}

pub fn outbound() -> OutboundData {
    OutboundData {
        inner: HashMap::new(),
    }
}
