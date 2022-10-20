#[cfg(test)]
mod discord_tests {
    use crate::discord::*;

    #[test]
    fn message() {
        let s = include_str!("discord_message.json").to_string();
        println!("{:#?}", inbound(s).unwrap());
    }
}
