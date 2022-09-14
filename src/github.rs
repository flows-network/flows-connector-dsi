use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Repository {
    name: String,
    full_name: String,
    private: bool,
    owner: User,
    html_url: String,
    description: Option<String>,
    fork: bool,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: Option<String>,
    stargazers_count: u32,
    watchers_count: u32,
    language: Option<String>,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    forks_count: u32,
    mirror_url: Option<String>,
    archived: bool,
    disabled: bool,
    open_issues_count: u32,
    license: Option<String>,
    forks: u32,
    open_issues: u32,
    watchers: u32,
    default_branch: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Organization {
    login: String,
    avatar_url: String,
    description: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct User {
    login: String,
    avatar_url: String,
    html_url: String,
    r#type: String,
    site_admin: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DiscussionCategory {
    name: String,
    description: String,
    emoji: Option<String>,
    created_at: String,
    updated_at: String,
    slug: Option<String>,
    is_answerable: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Comment {
    html_url: String,
    user: User,
    position: Option<String>,
    path: Option<String>,
    created_at: String,
    updated_at: String,
    author_association: String,
    body: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Discussion {
    category: DiscussionCategory,
    answer_html_url: Option<String>,
    answer_chosen_at: Option<String>,
    answer_chosen_by: Option<String>,
    html_url: String,
    user: User,
    number: u32,
    title: String,
    state: String,
    locked: bool,
    comments: u32,
    created_at: String,
    updated_at: String,
    author_association: String,
    active_lock_reason: Option<String>,
    body: String,
}

type Fork = Repository;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Label {
    name: String,
    color: String,
    default: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Issue {
    html_url: String,
    number: u32,
    title: String,
    user: User,
    labels: Vec<Label>,
    state: String,
    locked: bool,
    assignees: Vec<User>,
    comments: u32,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    author_association: String,
    body: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Plan {
    name: String,
    description: String,
    monthly_price_in_cents: u32,
    yearly_price_in_cents: u32,
    price_model: String,
    has_free_trial: bool,
    unit_name: String,
    bullets: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct MarketplacePurchase {
    billing_cycle: String,
    unit_count: u32,
    on_free_trial: bool,
    free_trial_ends_on: Option<String>,
    next_billing_date: String,
    plan: Plan,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Commit {
    label: String,
    r#ref: String,
    sha: String,
    user: User,
    repo: Repository
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PullRequest {
    html_url: String,
    diff_url: String,
    patch_url: String,
    number: u32,
    state: String,
    locked: bool,
    title: String,
    user: User,
    body: String,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    merged_at: Option<String>,
    merge_commit_sha: Option<String>,
    assignees: Vec<User>,
    requested_reviewers: Vec<User>,
    labels: Vec<Label>,
    head: Commit,
    base: Commit,
    author_association: String,
    draft: Option<bool>,
    merged: Option<bool>,
    mergeable: Option<bool>,
    mergeable_state: Option<String>,
    merged_by: Option<User>,
    comments: Option<u32>,
    review_comments: Option<u32>,
    maintainer_can_modify: Option<bool>,
    commits: Option<u32>,
    additions: Option<u32>,
    deletions: Option<u32>,
    changed_files: Option<u32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Review {
    user: User,
    body: Option<String>,
    commit_id: String,
    submitted_at: String,
    state: String,
    html_url: String,
    author_association: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ReleaseAsset {
    browser_download_url: String,
    name: String,
    label: Option<String>,
    state: String,
    content_type: String,
    size: i64,
    download_count: i64,
    created_at: String,
    updated_at: String,
    uploader: User,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Release {
    html_url: String,
    tag_name: String,
    target_commitish: String,
    name: Option<String>,
    draft: bool,
    author: User,
    assets: Vec<ReleaseAsset>,
    prerelease: bool,
    created_at: String,
    published_at: String,
    body: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WorkflowStep {
    name: String,
    status: String,
    conclusion: Option<String>,
    number: u32,
    started_at: String,
    completed_at: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WorkflowJob {
    html_url: String,
    status: String,
    conclusion: Option<String>,
    started_at: String,
    completed_at: Option<String>,
    name: String,
    steps: Vec<WorkflowStep>,
    labels: Vec<String>,
    runner_id: u32,
    runner_name: String,
    runner_group_id: u32,
    runner_group_name: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct InboundData {
    action: Option<String>,
    sender: User,
    repository: Option<Repository>,
    organization: Option<Organization>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
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
