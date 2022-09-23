use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct OutboundData {
    personalizations: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,

    content: Value,
}

impl OutboundData {
    /// Set the content of email, this field is required.
    pub fn content<S: Into<String> + Serialize>(mut self, content: S) -> OutboundData {
        self.content = json!([
            {
                "type": "text/html",
                "value": content.into(),
            }
        ]);
        self
    }

    /// Set the subject of email, this field is optional.
    pub fn subject<S: Into<String>>(mut self, subject: S) -> OutboundData {
        self.subject = Some(subject.into());
        self
    }

    /// Build outbound JSON data.
    pub fn build(self) -> Result<String, String> {
        if self.content == Value::Null {
            return Err("OutboundData build failed: Content is empty".to_string());
        }

        serde_json::to_string(&vec![self])
            .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
    }
}

/// Send an email to multiple email addresses via SendGrid.
/// 
/// eg.
/// ```rust
/// outbound(vec!["ho-229@example.com"])
///     .subject("Hi")
///     .content("Hello world!")
///     .build()
/// ```
pub fn outbound<S: Into<String> + Serialize>(emails: Vec<S>) -> OutboundData {
    let emails = emails
        .into_iter()
        .map(|e| json!({ "email": e }))
        .collect::<Vec<_>>();

    OutboundData {
        personalizations: json!([{ "to": emails }]),
        subject: None,
        content: Value::default(),
    }
}
