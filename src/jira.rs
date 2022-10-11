use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChangeLogItem {
    pub field: String,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub fieldtype: String,
    pub from: Option<String>,
    #[serde(rename = "fromString")]
    pub from_string: Option<String>,
    pub to: Option<String>,
    #[serde(rename = "toString")]
    pub to_string: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ChangeLog {
    pub items: Vec<ChangeLogItem>,
}

#[derive(Deserialize, Debug)]
pub struct AvatarUrls {
    #[serde(rename = "16x16")]
    _16x16: String,
    #[serde(rename = "24x24")]
    _24x24: String,
    #[serde(rename = "32x32")]
    _32x32: String,
    #[serde(rename = "48x48")]
    _48x48: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename = "accountType")]
    pub account_type: String,
    pub active: bool,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "self")]
    pub _self: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: String,
    #[serde(rename = "self")]
    pub _self: String,
    pub simplified: bool,
}

#[derive(Deserialize, Debug)]
pub struct Progress {
    pub progress: u64,
    pub total: u64,
}

#[derive(Deserialize, Debug)]
pub struct IssueType {
    pub description: String,
    pub name: String,
    #[serde(rename = "self")]
    pub _self: String,
    pub subtask: bool,
}

#[derive(Deserialize, Debug)]
pub struct Votes {
    #[serde(rename = "hasVoted")]
    pub has_voted: bool,
    #[serde(rename = "self")]
    pub _self: String,
    pub votes: u64,
}

#[derive(Deserialize, Debug)]
pub struct IssueField {
    pub aggregateprogress: Option<Progress>,
    pub assignee: Option<User>,
    pub created: Option<String>,
    pub creator: Option<User>,
    #[serde(default = "Vec::new")]
    pub labels: Vec<String>,
    #[serde(rename = "lastViewed")]
    pub last_viewed: Option<String>,
    pub progress: Option<Progress>,
    pub project: Project,
    pub reporter: Option<User>,
    pub summary: String,
    pub updated: Option<String>,
    pub votes: Option<Votes>,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub fields: IssueField,
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub _self: String,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub author: User,
    pub body: String,
    pub created: String,
    #[serde(rename = "self")]
    pub _self: String,
    #[serde(rename = "updateAuthor")]
    pub update_author: User,
    pub updated: String,
}

#[derive(Deserialize, Debug)]
pub struct InboundData {
    pub changelog: Option<ChangeLog>,
    pub issue: Option<Issue>,
    pub comment: Option<Comment>,
    pub timestamp: u64,
    #[serde(rename = "webhookEvent")]
    pub webhook_event: String,
}

impl InboundData {
    #[inline]
    pub fn get_changelog(&self) -> Result<&ChangeLog, String> {
        self.changelog
            .as_ref()
            .ok_or("Missing changelog".to_string())
    }

    #[inline]
    pub fn get_issue(&self) -> Result<&Issue, String> {
        self.issue.as_ref().ok_or("Missing issue".to_string())
    }

    #[inline]
    pub fn get_comment(&self) -> Result<&Comment, String> {
        self.comment.as_ref().ok_or("Missing comment".to_string())
    }
}

/// Deserialize the Jira Webhook Payload into `InboundData`.
pub fn inbound(s: String) -> Result<InboundData, String> {
    #[cfg(debug_assertions)]
    return serde_json::from_str::<InboundData>(&s)
        .map_err(|e| format!("Parsing Jira Webhook payload failed: {}", e.to_string()));

    #[cfg(not(debug_assertions))]
    serde_json::from_str::<InboundData>(&s)
        .map_err(|_| format!("Parsing Jira Webhook payload failed: {}", s))
}

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
        pub fn description<S: Into<String>>(mut self, description: S) -> OutboundData<'a> {
            self.inner.insert("description", json!(description.into()));
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
