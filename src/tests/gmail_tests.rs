#[cfg(test)]
mod tests {
    use crate::gmail::outbound;

    #[test]
    fn gmail_test() {
        let body = outbound("example@example.com")
            .sender("ho-229")
            .subject("Test")
            .content("This is a test message.\n")
            .build()
            .unwrap();

        assert_eq!(&body, "From: me\r\nTo: example@example.com\r\nSender: ho-229\r\nSubject: Test\r\n\r\nThis is a test message.\r\n");
    }
}
