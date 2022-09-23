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
    /// Sets the value of `Status` column type with index number.
    pub fn status_index<C: Into<String>>(mut self, column_id: C, index: u32) -> OutboundData {
        self.values.insert(column_id.into(), json!(index));
        self
    }

    /// Sets the value of `Status` column type with label string.
    pub fn status_label<C: Into<String>, L: Into<String>>(
        mut self,
        column_id: C,
        label: L,
    ) -> OutboundData {
        self.values.insert(column_id.into(), json!(label.into()));
        self
    }

    /// Sets the value of `Text` column type.
    pub fn text<C: Into<String>, T: Into<String>>(mut self, column_id: C, text: T) -> OutboundData {
        self.values.insert(column_id.into(), json!(text.into()));
        self
    }

    /// Sets the value of `Number` column type.
    pub fn number<C: Into<String>, T: Into<String>>(
        mut self,
        column_id: C,
        number: f64,
    ) -> OutboundData {
        self.values
            .insert(column_id.into(), json!(number.to_string()));
        self
    }

    /// Sets the value of `Date` column type.
    pub fn date<C: Into<String>>(mut self, column_id: C, date: NaiveDate) -> OutboundData {
        self.values
            .insert(column_id.into(), json!(date.format("%Y-%m-%d").to_string()));
        self
    }

    /// Sets the value of `Timeline` column type.
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

    /// Sets the value of `Person` column type.
    pub fn person<C: Into<String>, P: Into<String>>(
        mut self,
        column_id: C,
        person_id: P,
    ) -> OutboundData {
        self.values
            .insert(column_id.into(), json!(person_id.into()));
        self
    }

    /// Sets the value of other unsupported column.
    pub fn value<C: Into<String>>(mut self, column_id: C, value: Value) -> OutboundData {
        self.values.insert(column_id.into(), value);
        self
    }

    /// Build outbound JSON data.
    pub fn build(self) -> Result<String, String> {
        if self.values.is_empty() {
            return Err("OutboundData build failed: values is empty".to_string());
        }

        serde_json::to_string(&self)
            .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
    }
}

/// Updates the values of some specified columns of an item.
/// 
/// eg.
/// ```rust
/// outbound("item #1")
///     .status_label("status", "Opened")     // .status_index("status", 1)
///     .date("date4", NaiveDate::from_ymd(1970, 1, 1))
///     .text("text", "text")
///     .number("number4", 3.141)
///     .build()
/// ```
pub fn outbound<S: Into<String>>(item: S) -> OutboundData {
    OutboundData {
        item: item.into(),
        values: HashMap::new(),
    }
}
