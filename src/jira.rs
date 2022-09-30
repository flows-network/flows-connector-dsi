pub mod outbound {
    use std::collections::HashMap;

    use serde_json::{json, Value};

    pub struct OutboundData<'a> {
        inner: HashMap<&'a str, Value>,
    }

    impl<'a> OutboundData<'a> {
        /// Set summary.
        pub fn summary<S: Into<String>>(mut self, summary: S) -> OutboundData<'a> {
            self.inner.insert("summary", json!(summary.into()));
            self
        }

        /// Set description.
        pub fn description<S: Into<String>>(mut self, paragraphs: Vec<S>) -> OutboundData<'a> {
            self.inner.insert(
                "description",
                json!({
                    "type": "doc",
                    "version": "1",
                    "content": paragraphs
                        .into_iter()
                        .map(|paragraph| json!({
                            "type": "paragraph",
                            "content": [
                                {
                                    "text": paragraph.into(),
                                    "type": "text"
                                }
                            ]
                        }))
                        .collect::<Vec<_>>()
                }),
            );
            self
        }

        /// Set transition, only used on modify issue.
        pub fn transition<S: Into<String>>(mut self, transition: S) -> OutboundData<'a> {
            self.inner.insert("transition", json!(transition.into()));
            self
        }

        /// Build outbound JSON data.
        pub fn build(self) -> Result<String, String> {
            if self.inner.is_empty() {
                return Err("OutboundData build failed: empty field".to_string());
            }

            serde_json::to_string(&self.inner)
                .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
        }
    }

    /// Create an issue with the summary.
    pub fn create_issue<'a, S: Into<String>>(summary: S) -> OutboundData<'a> {
        OutboundData {
            inner: [("summary", json!(summary.into()))]
                .into_iter()
                .collect::<HashMap<&str, Value>>(),
        }
    }

    /// Modified an issue like change the summary, description or transition.
    pub fn modify_issue<'a, S: Into<String>>(issue_key: S) -> OutboundData<'a> {
        OutboundData {
            inner: [("issue_key", json!(issue_key.into()))]
                .into_iter()
                .collect::<HashMap<&str, Value>>(),
        }
    }
}
