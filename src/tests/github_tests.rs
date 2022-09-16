#[cfg(test)]
mod github_tests {
    use serde_json::{json, Value};

    use crate::github::*;

    #[test]
    fn create_test() {
        let s = include_str!("github_create.json").to_string();

        println!("{:#?}", inbound(s).unwrap());
    }

    #[test]
    fn discussion_comment_test() {
        let s = include_str!("github_discussion_comment.json").to_string();
        let d = inbound(s).unwrap();

        println!(
            "{:#?}\n{:#?}",
            d.get_comment().unwrap(),
            d.get_discussion().unwrap()
        );
    }

    #[test]
    fn fork_test() {
        let s = include_str!("github_fork.json").to_string();

        println!("{:#?}", inbound(s).unwrap().get_fork().unwrap());
    }

    #[test]
    fn issue_comment_test() {
        let s = include_str!("github_issue_comment.json").to_string();
        let d = inbound(s).unwrap();

        println!(
            "{:#?}\n{:#?}",
            d.get_comment().unwrap(),
            d.get_issue().unwrap()
        );
    }

    #[test]
    fn label_test() {
        let s = include_str!("github_label.json").to_string();

        println!("{:#?}", inbound(s).unwrap().get_label().unwrap());
    }

    #[test]
    fn marketplace_purchase_test() {
        let s = include_str!("github_marketplace_purchase.json").to_string();

        println!(
            "{:#?}",
            inbound(s).unwrap().get_marketplace_purchase().unwrap()
        );
    }

    #[test]
    fn pull_request_test() {
        let s = include_str!("github_pull_request_review.json").to_string();
        let d = inbound(s).unwrap();

        println!(
            "{:#?}\n{:#?}",
            d.get_review().unwrap(),
            d.get_pull_request().unwrap()
        );
    }

    #[test]
    fn release_test() {
        let s = include_str!("github_release.json").to_string();

        println!("{:#?}", inbound(s).unwrap().get_release().unwrap());
    }

    #[test]
    fn workflow_job_test() {
        let s = include_str!("github_workflow_job.json").to_string();

        println!("{:#?}", inbound(s).unwrap().get_workflow_job().unwrap());
    }

    #[test]
    fn push_test() {
        let s = include_str!("github_push.json").to_string();
        let d = inbound(s).unwrap();

        println!(
            "{:#?}\n{:#?}",
            d.get_head_commit().unwrap(),
            d.get_commits().unwrap()
        );
    }

    #[test]
    fn outbound_test() {
        let d = outbound::create_issue("issue title")
            .labels(vec!["test"])
            .milestone(json!(1))
            .assignees(vec!["ho-229"])
            .body("message")
            .build()
            .unwrap();

        assert_eq!(
            serde_json::from_str::<Value>(&d).unwrap(),
            json!({
                "title": "issue title",
                "labels": ["test"],
                "milestone": 1,
                "assignees": ["ho-229"],
                "body": "message"
            })
        );
    }
}
