pub struct OutboundData {
    fields: String,
    content: String,
}

impl OutboundData {
    /// Set content of email, this field is required.
    pub fn content<S: Into<String>>(mut self, content: S) -> OutboundData {
        self.content = content.into();
        self
    }

    /// Set sender of email, this field is optional.
    pub fn sender<S: Into<String>>(mut self, sender: S) -> OutboundData {
        self.fields
            .push_str(&format!("Sender: {}\r\n", sender.into()));
        self
    }

    /// Set subject of email, this field is optional.
    pub fn subject<S: Into<String>>(mut self, subject: S) -> OutboundData {
        self.fields
            .push_str(&format!("Subject: {}\r\n", subject.into()));
        self
    }

    /// Build outbound data in `RFC 2822` form.
    pub fn build(self) -> Result<String, String> {
        if self.content.is_empty() {
            return Err("OutboundData build failed: Content is empty".to_string());
        }

        Ok(format!(
            "{}\r\n{}",
            self.fields,
            self.content.replace("\n", "\r\n")
        ))
    }
}

/// Send an email via GMail.
///
/// ```rust
/// outbound("ho-229@example.com")
///     .subject("Hi")
///     .sender("ho-229")
///     .content("Hello world!")
///     .build()
/// ```
pub fn outbound<S: Into<String>>(email: S) -> OutboundData {
    OutboundData {
        fields: format!("From: me\r\nTo: {}\r\n", email.into()),
        content: String::new(),
    }
}
