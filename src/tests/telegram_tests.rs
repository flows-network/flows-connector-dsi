#[cfg(test)]
mod telegram_tests {
    use crate::telegram::*;

    #[test]
    fn message() {
        let s = include_str!("telegram_message.json").to_string();

        println!("{:#?}", inbound(s).unwrap());
    }

    #[test]
    fn my_chat_member() {
        let s = include_str!("telegram_my_chat_member.json").to_string();

        println!("{:#?}", inbound(s).unwrap());
    }
}
