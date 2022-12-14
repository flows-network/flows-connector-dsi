use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum License {
    Object {
        key: String,
        name: String,
        spdx_id: String,
        url: String,
    },
    Name(String),
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: User,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub created_at: Value,
    pub updated_at: String,
    pub pushed_at: Value,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: u32,
    pub mirror_url: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: u32,
    pub license: Option<License>,
    pub visibility: String,
    pub forks: u32,
    pub open_issues: u32,
    pub watchers: u32,
    pub default_branch: String,
}

#[derive(Deserialize, Debug)]
pub struct Organization {
    pub login: String,
    pub avatar_url: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub email: Option<String>,
    pub avatar_url: String,
    pub html_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Deserialize, Debug)]
pub struct DiscussionCategory {
    pub name: String,
    pub description: String,
    pub emoji: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub slug: Option<String>,
    pub is_answerable: bool,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub html_url: String,
    pub user: User,
    pub position: Option<u32>,
    pub path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct Discussion {
    pub category: DiscussionCategory,
    pub answer_html_url: Option<String>,
    pub answer_chosen_at: Option<String>,
    pub answer_chosen_by: Option<String>,
    pub html_url: String,
    pub user: User,
    pub number: u32,
    pub title: String,
    pub state: String,
    pub locked: bool,
    pub comments: u32,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub active_lock_reason: Option<String>,
    pub body: String,
}

type Fork = Repository;

#[derive(Deserialize, Debug)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub default: bool,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub active_lock_reason: Option<String>,
    pub html_url: String,
    pub number: u32,
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub state_reason: Option<String>,
    pub locked: bool,
    pub assignees: Vec<User>,
    pub comments: u32,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: String,
    pub body: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Plan {
    pub name: String,
    pub description: String,
    pub monthly_price_in_cents: u32,
    pub yearly_price_in_cents: u32,
    pub price_model: String,
    pub has_free_trial: bool,
    pub unit_name: String,
    pub bullets: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct MarketplacePurchase {
    pub billing_cycle: String,
    pub unit_count: u32,
    pub on_free_trial: bool,
    pub free_trial_ends_on: Option<String>,
    pub next_billing_date: String,
    pub plan: Plan,
}

#[derive(Deserialize, Debug)]
pub struct PrPoint {
    pub label: String,
    pub r#ref: String,
    pub sha: String,
    pub user: User,
    pub repo: Repository,
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub number: u32,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: User,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    pub labels: Vec<Label>,
    pub head: PrPoint,
    pub base: PrPoint,
    pub author_association: String,
    pub draft: Option<bool>,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<User>,
    pub comments: Option<u32>,
    pub review_comments: Option<u32>,
    pub maintainer_can_modify: Option<bool>,
    pub commits: Option<u32>,
    pub additions: Option<u32>,
    pub deletions: Option<u32>,
    pub changed_files: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Review {
    pub user: User,
    pub body: Option<String>,
    pub commit_id: String,
    pub submitted_at: String,
    pub state: String,
    pub html_url: String,
    pub author_association: String,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseAsset {
    pub browser_download_url: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub uploader: User,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub html_url: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub author: User,
    pub assets: Vec<ReleaseAsset>,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub body: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowStep {
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub number: u32,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowJob {
    pub html_url: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub labels: Vec<String>,
    pub runner_id: Option<u32>,
    pub runner_name: Option<String>,
    pub runner_group_id: Option<u32>,
    pub runner_group_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub distinct: bool,
    #[serde(default)]
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: GitUser,
    pub committer: GitUser,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct InboundData {
    pub sender: User,

    pub action: Option<String>,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub comment: Option<Comment>,
    pub discussion: Option<Discussion>,
    pub forkee: Option<Fork>,
    pub issue: Option<Issue>,
    pub label: Option<Label>,
    pub marketplace_purchase: Option<MarketplacePurchase>,
    pub pull_request: Option<PullRequest>,
    pub review: Option<Review>,
    pub release: Option<Release>,
    pub starred_at: Option<String>,
    pub workflow_job: Option<WorkflowJob>,
    pub head_commit: Option<Commit>,
    pub commits: Option<Vec<Commit>>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl InboundData {
    /// Gets the value from `extra` and deserializes into `T`.
    ///
    /// eg.
    /// ```rust
    /// let r#ref: String = payload.get("ref").unwrap();
    /// ```
    #[inline]
    pub fn get<T: DeserializeOwned, I: ToString>(&self, index: &I) -> Result<T, String> {
        serde_json::from_value(
            self.extra
                .get(&index.to_string())
                .ok_or(format!("Missing {}", index.to_string()))?
                .clone(),
        )
        .map_err(|e| e.to_string())
    }

    #[inline]
    pub fn get_action(&self) -> Result<&String, String> {
        self.action.as_ref().ok_or("Missing action".to_string())
    }

    #[inline]
    pub fn get_repository(&self) -> Result<&Repository, String> {
        self.repository
            .as_ref()
            .ok_or("Missing repository".to_string())
    }

    #[inline]
    pub fn get_comment(&self) -> Result<&Comment, String> {
        self.comment.as_ref().ok_or("Missing comment".to_string())
    }

    #[inline]
    pub fn get_discussion(&self) -> Result<&Discussion, String> {
        self.discussion
            .as_ref()
            .ok_or("Missing discussion".to_string())
    }

    #[inline]
    pub fn get_fork(&self) -> Result<&Fork, String> {
        self.forkee.as_ref().ok_or("Missing forkee".to_string())
    }

    #[inline]
    pub fn get_issue(&self) -> Result<&Issue, String> {
        self.issue.as_ref().ok_or("Missing issue".to_string())
    }

    #[inline]
    pub fn get_label(&self) -> Result<&Label, String> {
        self.label.as_ref().ok_or("Missing label".to_string())
    }

    #[inline]
    pub fn get_marketplace_purchase(&self) -> Result<&MarketplacePurchase, String> {
        self.marketplace_purchase
            .as_ref()
            .ok_or("Missing marketplace_purchase".to_string())
    }

    #[inline]
    pub fn get_pull_request(&self) -> Result<&PullRequest, String> {
        self.pull_request
            .as_ref()
            .ok_or("Missing pull_request".to_string())
    }

    #[inline]
    pub fn get_review(&self) -> Result<&Review, String> {
        self.review.as_ref().ok_or("Missing review".to_string())
    }

    #[inline]
    pub fn get_release(&self) -> Result<&Release, String> {
        self.release.as_ref().ok_or("Missing release".to_string())
    }

    #[inline]
    pub fn get_starred_at(&self) -> Result<&String, String> {
        self.starred_at
            .as_ref()
            .ok_or("Missing starred_at".to_string())
    }

    #[inline]
    pub fn get_workflow_job(&self) -> Result<&WorkflowJob, String> {
        self.workflow_job
            .as_ref()
            .ok_or("Missing workflow_job".to_string())
    }

    pub fn get_head_commit(&self) -> Result<&Commit, String> {
        self.head_commit
            .as_ref()
            .ok_or("Missing head_commit".to_string())
    }

    pub fn get_commits(&self) -> Result<&Vec<Commit>, String> {
        self.commits.as_ref().ok_or("Missing commits".to_string())
    }
}

/// Deserialize the GitHub Webhook Payload into `InboundData`.
pub fn inbound(s: String) -> Result<InboundData, String> {
    #[cfg(debug_assertions)]
    return serde_json::from_str::<InboundData>(&s)
        .map_err(|e| format!("Parsing GitHub Webhook payload failed: {}", e.to_string()));

    #[cfg(not(debug_assertions))]
    serde_json::from_str::<InboundData>(&s)
        .map_err(|_| format!("Parsing GitHub Webhook payload failed: {}", s))
}

pub mod outbound {
    use std::collections::HashMap;

    use serde::Serialize;
    use serde_json::{json, Value};

    #[derive(Serialize)]
    pub struct OutboundData<'a> {
        #[serde(flatten)]
        inner: HashMap<&'a str, Value>,
    }

    impl<'a> OutboundData<'a> {
        /// Set the issue or issue comment body.
        pub fn body<S: ToString + Serialize>(mut self, body: S) -> OutboundData<'a> {
            self.inner.insert("body", json!(body));
            self
        }

        /// Set Milestone.
        pub fn milestone(mut self, milestone: Value) -> OutboundData<'a> {
            self.inner.insert("milestone", milestone);
            self
        }

        /// Set labels.
        pub fn labels<S: ToString + Serialize>(mut self, labels: Vec<S>) -> OutboundData<'a> {
            self.inner.insert("labels", json!(labels));
            self
        }

        /// Set assignees.
        pub fn assignees<S: ToString + Serialize>(mut self, assignees: Vec<S>) -> OutboundData<'a> {
            self.inner.insert("assignees", json!(assignees));
            self
        }

        /// Build outbound JSON data.
        pub fn build(self) -> Result<String, String> {
            if self.inner.len() < 2 {
                return Err("OutboundData build failed: Too few fields".to_string());
            }

            serde_json::to_string(&self)
                .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
        }
    }

    /// Create an issue with the title.
    pub fn create_issue<'a, S: ToString + Serialize>(title: S) -> OutboundData<'a> {
        OutboundData {
            inner: [("title", json!(title))]
                .into_iter()
                .collect::<HashMap<&str, Value>>(),
        }
    }

    /// Modified an issue like change the assignees or labels,
    /// create a comment, etc.
    pub fn modify_issue<'a>(issue_number: u32) -> OutboundData<'a> {
        OutboundData {
            inner: [("issue_number", json!(issue_number))]
                .into_iter()
                .collect::<HashMap<&str, Value>>(),
        }
    }

    /// Merge a pull request.
    pub fn merge_pull<'a>(pull_number: u32) -> OutboundData<'a> {
        OutboundData {
            inner: [("pull_number", json!(pull_number))]
                .into_iter()
                .collect::<HashMap<&str, Value>>(),
        }
    }
}
