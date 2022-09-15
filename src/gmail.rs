pub struct OutboundData {
    fields: String,
    content: String,
}

impl OutboundData {
    pub fn content<S: Into<String>>(mut self, content: S) -> OutboundData {
        self.content = content.into();
        self
    }

    pub fn sender<S: Into<String>>(mut self, sender: S) -> OutboundData {
        self.fields.push_str(&format!("Sender: {}\r\n", sender.into()));
        self
    }

    pub fn subject<S: Into<String>>(mut self, subject: S) -> OutboundData {
        self.fields.push_str(&format!("Subject: {}\r\n", subject.into()));
        self
    }

    pub fn build(self) -> Result<String, String> {
        Ok(format!("{}\r\n{}", self.fields, self.content.replace("\n", "\r\n")))
    }
}

pub fn outbound<S: Into<String>>(email: S) -> OutboundData {
    OutboundData {
        fields: format!("From: me\r\nTo: {}\r\n", email.into()),
        content: String::new(),
    }
}
