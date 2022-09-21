pub use chrono::NaiveDate;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct OutboundData {
    item: String,
    values: HashMap<String, Value>,
}

impl OutboundData {
    pub fn status_index<C: Into<String>>(mut self, column_id: C, index: u32) -> OutboundData {
        self.values.insert(column_id.into(), json!(index));
        self
    }

    pub fn status_label<C: Into<String>, L: Into<String>>(
        mut self,
        column_id: C,
        label: L,
    ) -> OutboundData {
        self.values.insert(column_id.into(), json!(label.into()));
        self
    }

    pub fn text<C: Into<String>, T: Into<String>>(mut self, column_id: C, text: T) -> OutboundData {
        self.values.insert(column_id.into(), json!(text.into()));
        self
    }

    pub fn number<C: Into<String>, T: Into<String>>(
        mut self,
        column_id: C,
        number: f64,
    ) -> OutboundData {
        self.values
            .insert(column_id.into(), json!(number.to_string()));
        self
    }

    pub fn date<C: Into<String>>(mut self, column_id: C, date: NaiveDate) -> OutboundData {
        self.values
            .insert(column_id.into(), json!(date.format("%Y-%m-%d").to_string()));
        self
    }

    pub fn timeline<C: Into<String>>(
        mut self,
        column_id: C,
        from: NaiveDate,
        to: NaiveDate,
    ) -> OutboundData {
        self.values.insert(
            column_id.into(),
            json!({
                "from": from.format("%Y-%m-%d").to_string(),
                "to": to.format("%Y-%m-%d").to_string()
            }),
        );
        self
    }

    pub fn person<C: Into<String>, P: Into<String>>(
        mut self,
        column_id: C,
        person_id: P,
    ) -> OutboundData {
        self.values
            .insert(column_id.into(), json!(person_id.into()));
        self
    }

    pub fn value<C: Into<String>>(mut self, column_id: C, value: Value) -> OutboundData {
        self.values.insert(column_id.into(), value);
        self
    }

    pub fn build(self) -> Result<String, String> {
        if self.values.is_empty() {
            return Err("OutboundData build failed: values is empty".to_string());
        }

        serde_json::to_string(&self)
            .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
    }
}

pub fn outbound<S: Into<String>>(item: S) -> OutboundData {
    OutboundData {
        item: item.into(),
        values: HashMap::new(),
    }
}
