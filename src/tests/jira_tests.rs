#[cfg(test)]
mod jira_tests {
    use crate::jira::*;

    #[test]
    fn issue_created_test() {
        let s = include_str!("jira_issue_created.json").to_string();
        println!("{:#?}", inbound(s).unwrap().get_issue().unwrap());
    }

    #[test]
    fn issue_updated_test() {
        let s = include_str!("jira_issue_updated.json").to_string();
        println!("{:#?}", inbound(s).unwrap().get_issue().unwrap());
    }

    #[test]
    fn comment_created_test() {
        let s = include_str!("jira_comment_created.json").to_string();
        println!("{:#?}", inbound(s).unwrap().get_comment().unwrap());
    }

    #[test]
    fn comment_updated_test() {
        let s = include_str!("jira_comment_updated.json").to_string();
        println!("{:#?}", inbound(s).unwrap().get_comment().unwrap());
    }
}
