use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: User,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
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
    pub license: Option<String>,
    pub forks: u32,
    pub open_issues: u32,
    pub watchers: u32,
    pub default_branch: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Organization {
    pub login: String,
    pub avatar_url: String,
    pub description: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Comment {
    pub html_url: String,
    pub user: User,
    pub position: Option<String>,
    pub path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub body: String,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub default: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Issue {
    pub html_url: String,
    pub number: u32,
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignees: Vec<User>,
    pub comments: u32,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: String,
    pub body: String,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct MarketplacePurchase {
    pub billing_cycle: String,
    pub unit_count: u32,
    pub on_free_trial: bool,
    pub free_trial_ends_on: Option<String>,
    pub next_billing_date: String,
    pub plan: Plan,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Commit {
    pub label: String,
    pub r#ref: String,
    pub sha: String,
    pub user: User,
    pub repo: Repository
}

#[allow(dead_code)]
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
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    pub labels: Vec<Label>,
    pub head: Commit,
    pub base: Commit,
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WorkflowStep {
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub number: u32,
    pub started_at: String,
    pub completed_at: Option<String>,
}

#[allow(dead_code)]
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
    pub runner_id: u32,
    pub runner_name: String,
    pub runner_group_id: u32,
    pub runner_group_name: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct InboundData {
    pub action: Option<String>,
    pub sender: User,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl InboundData {
    pub fn get_from_extra<T: DeserializeOwned>(&self, name: &str) -> Result<T, String> {
        serde_json::from_value(
            self.extra
                .get(name)
                .ok_or(format!("Missing {}.", name))?
                .clone(),
        )
        .map_err(|e| e.to_string())
    }

    #[inline]
    pub fn get_comment(&self) -> Result<Comment, String> {
        self.get_from_extra("comment")
    }

    #[inline]
    pub fn get_discussion(&self) -> Result<Discussion, String> {
        self.get_from_extra("discussion")
    }

    #[inline]
    pub fn get_fork(&self) -> Result<Fork, String> {
        self.get_from_extra("forkee")
    }

    #[inline]
    pub fn get_issue(&self) -> Result<Issue, String> {
        self.get_from_extra("issue")
    }

    #[inline]
    pub fn get_label(&self) -> Result<Label, String> {
        self.get_from_extra("label")
    }

    #[inline]
    pub fn get_marketplace_purchase(&self) -> Result<MarketplacePurchase, String> {
        self.get_from_extra("marketplace_purchase")
    }

    #[inline]
    pub fn get_pull_request(&self) -> Result<PullRequest, String> {
        self.get_from_extra("pull_request")
    }

    #[inline]
    pub fn get_review(&self) -> Result<Review, String> {
        self.get_from_extra("review")
    }

    #[inline]
    pub fn get_release(&self) -> Result<Release, String> {
        self.get_from_extra("release")
    }

    #[inline]
    pub fn get_starred_at(&self) -> Result<String, String> {
        self.get_from_extra("starred_at")
    }

    #[inline]
    pub fn get_workflow_job(&self) -> Result<WorkflowJob, String> {
        self.get_from_extra("workflow_job")
    }
}

pub fn inbound(s: String) -> Result<InboundData, String> {
    serde_json::from_str::<InboundData>(&s).map_err(|e| e.to_string())
}

pub mod outbound {
    use serde_json::{json, Value};

    pub fn create_issue(
        title: String,
        body: String,
        milestone: Value,       // null or string or integer
        labels: Vec<String>,
        assignees: Vec<String>,
    ) -> String {
        json!({
            "title": title,
            "body": body,
            "milestone": milestone,
            "labels": labels,
            "assignees": assignees,
        })
        .to_string()
    }

    pub fn create_comment(
        issue_number: u32,
        body: String,
    ) -> String {
        json!({
            "issue_number": issue_number,
            "body": body,
        })
        .to_string()
    }

    pub fn add_labels(
        issue_number: u32,
        labels: Vec<String>,
    ) -> String {
        json!({
            "issue_number": issue_number,
            "labels": labels,
        })
        .to_string()
    }

    pub fn add_assignees(
        issue_number: u32,
        assignees: Vec<String>,
    ) -> String {
        json!({
            "issue_number": issue_number,
            "assignees": assignees,
        })
        .to_string()
    }
}
