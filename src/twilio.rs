use serde::Serialize;

#[derive(Serialize)]
pub struct OutboundData {
    #[serde(rename = "To")]
    to: String,
    #[serde(rename = "Body")]
    body: Option<String>,
}

impl OutboundData {
    /// Set the body.
    pub fn body<S: Into<String>>(mut self, body: S) -> OutboundData {
        self.body = Some(body.into());
        self
    }

    /// Build outbound JSON data.
    pub fn build(self) -> Result<String, String> {
        if self.body.is_none() {
            return Err("OutboundData build failed: Body is empty".to_string());
        }

        serde_json::to_string(&self)
            .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
    }
}

/// Send a SMS message via Twilio.
/// 
/// eg.
/// ```rust
/// outbound("+11234567890")
///     .body("This is a test message")
///     .build()
/// ```
pub fn outbound<S: Into<String>>(phone_number: S) -> OutboundData {
    OutboundData {
        to: phone_number.into(),
        body: None,
    }
}
