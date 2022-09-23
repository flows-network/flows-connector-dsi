use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct OutboundData {
    #[serde(flatten)]
    inner: HashMap<String, String>,
}

impl OutboundData {
    /// Sets the content of specified property, 
    /// it will create a new property if it doesn't already exists
    pub fn property<P: Into<String>, C: Into<String>>(
        mut self,
        property: P,
        content: C,
    ) -> OutboundData {
        self.inner.insert(property.into(), content.into());
        self
    }

    #[deprecated(
        since = "0.1.4",
        note = "Please use property() instead of page(), this name can be confusing."
    )]
    pub fn page<P: Into<String>, C: Into<String>>(self, property: P, content: C) -> OutboundData {
        self.property(property, content)
    }

    /// Build outbound JSON data.
    pub fn build(self) -> Result<String, String> {
        if self.inner.is_empty() {
            return Err("OutboundData build failed: No properties to create a page".to_string());
        }

        serde_json::to_string(&self)
            .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
    }
}

/// Create a Notion database page with multiple properties.
///
/// eg.
/// ```rust
/// /*
/// | Name   | Email              |
/// | ------ | ------------------ |
/// | ho-229 | ho-229@example.com |
/// */
/// outbound()
///     .property("Name", "ho-229")
///     .property("Email", "ho-229@example.com")
///     .build()
/// ```
pub fn outbound() -> OutboundData {
    OutboundData::default()
}
