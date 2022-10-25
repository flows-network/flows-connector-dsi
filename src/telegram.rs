use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: u64,
    /// True, if this user is a bot.
    pub is_bot: bool,
    /// User's or bot's first name.
    pub first_name: String,
    /// User's or bot's username.
    pub username: Option<String>,
    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag).
    pub language_code: Option<String>,
    /// True, if this user is a Telegram Premium user.
    #[serde(default = "bool::default")]
    pub is_premium: bool,
    /// True, if this user added the bot to the attachment menu.
    #[serde(default = "bool::default")]
    pub added_to_attachment_menu: bool,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”.
    pub r#type: String,
    /// Title, for supergroups, channels and group chats.
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available.
    pub username: Option<String>,
    /// First name of the other party in a private chat.
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat.
    pub last_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MessageEntity {
    /// Type of the entity. Currently, can be “mention” (@username), “hashtag” (#hashtag),
    /// “cashtag” ($USD), “bot_command” (/start@jobs_bot), “url” (https://telegram.org),
    /// “email” (do-not-reply@telegram.org), “phone_number” (+1-212-555-0123),
    /// “bold” (bold text), “italic” (italic text), “underline” (underlined text),
    /// “strikethrough” (strikethrough text), “spoiler” (spoiler message),
    /// “code” (monowidth string), “pre” (monowidth block),
    /// “text_link” (for clickable text URLs), “text_mention” (for users without usernames),
    /// “custom_emoji” (for inline custom emoji stickers).
    pub r#type: String,
    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: i64,
    /// Length of the entity in UTF-16 code units.
    pub length: i64,
    /// For “text_link” only, URL that will be opened after user taps on the text.
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user.
    pub user: Option<User>,
    /// For “pre” only, the programming language of the entity text.
    pub language: Option<String>,
    // custom_emoji_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub message_id: i64,
    /// Conversation the message belongs to.
    pub chat: Chat,
    /// Date the message was sent in Unix time.
    pub date: u64,
    /// Sender of the message; empty for messages sent to channels.
    /// For backward compatibility, the field contains a fake sender user in non-channel chats,
    /// if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat.
    /// For example, the channel itself for channel posts,
    /// the supergroup itself for messages from anonymous group administrators,
    /// the linked channel for messages automatically forwarded to the discussion group.
    /// For backward compatibility,
    /// the field from contains a fake sender user in non-channel chats,
    /// if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// For text messages..
    #[serde(default = "String::new")]
    pub text: String,
    /// For messages forwarded from channels or from anonymous administrators,
    /// information about the original sender chat.
    pub forward_from: Option<User>,
    /// For messages forwarded from channels or from anonymous administrators,
    /// information about the original sender chat.
    pub forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels,
    /// identifier of the original message in the channel
    pub forward_from_message_id: Option<i64>,
    /// For forwarded messages, date the original message was sent in Unix time.
    pub forward_date: Option<u64>,
    /// For replies, the original message.
    /// Note that the Message object in this field will not contain further
    /// reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent.
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<u64>,
    /// New members that were added to the group or supergroup
    /// and information about them (the bot itself may be one of these members)
    #[serde(default = "Vec::new")]
    pub new_chat_members: Vec<User>,
    /// A member was removed from the group,
    /// information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    #[serde(default = "Vec::new")]
    pub entities: Vec<MessageEntity>,
}

#[derive(Deserialize, Debug)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link.
    pub creator: User,
    /// True, if users joining the chat via the link need
    /// to be approved by chat administrators.
    pub creates_join_request: bool,
    /// True, if the link is primary.
    pub is_primary: bool,
    /// True, if the link is revoked.
    pub is_revoked: bool,
    /// Optional. Invite link name.
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired.
    pub expire_date: Option<u64>,
    /// The maximum number of users that can be members of the chat simultaneously.
    /// after joining the chat via this invite link; 1-99999.
    pub member_limit: Option<u64>,
    /// Number of pending join requests created using this link.
    pub pending_join_request_count: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ChatMember {
    /// Status of the chat member. it can be “creator” (owner) , "administrator",
    /// "member", “restricted”, “left” (member who left), "kicked” (member who was banned) .
    pub status: String,
    /// Information about the user.
    pub user: User,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to.
    pub chat: Chat,
    /// Performer of the action, which resulted in the change.
    pub from: User,
    /// Date the change was done in Unix time.
    pub date: u64,
    /// Previous information about the chat member.
    pub old_chat_member: ChatMember,
    /// New information about the chat member.
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat;
    /// for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Deserialize, Debug)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent.
    pub chat: Chat,
    /// User that sent the join request.
    pub from: User,
    /// Date the request was sent in Unix time.
    pub date: u64,
    /// Bio of the user.
    #[serde(default = "String::new")]
    pub bio: String,
    /// Chat invite link that was used by the user to send the join request.
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Deserialize, Debug)]
pub enum InboundData {
    #[serde(rename = "message")]
    Message(Message),
    #[serde(rename = "edited_message")]
    EditedMessage(Message),
    #[serde(rename = "channel_post")]
    ChannelPost(Message),
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost(Message),
    #[serde(rename = "my_chat_member")]
    MyChatMember(ChatMemberUpdated),
    #[serde(rename = "chat_member")]
    ChatMember(ChatMemberUpdated),
    #[serde(rename = "chat_join_request")]
    ChatJoinRequest(ChatJoinRequest),
}

pub fn inbound(s: String) -> Result<InboundData, String> {
    #[cfg(debug_assertions)]
    return serde_json::from_str::<InboundData>(&s)
        .map_err(|e| format!("Parsing Telegram Webhook payload failed: {}", e.to_string()));

    #[cfg(not(debug_assertions))]
    serde_json::from_str::<InboundData>(&s)
        .map_err(|_| format!("Parsing Telegram Webhook payload failed: {}", s))
}

impl InboundData {
    pub fn as_message(&self) -> Result<&Message, String> {
        match self {
            InboundData::Message(ref m) => Ok(m),
            _ => Err("as_message failed".to_string()),
        }
    }

    pub fn as_edited_message(&self) -> Result<&Message, String> {
        match self {
            InboundData::EditedMessage(ref m) => Ok(m),
            _ => Err("as_edited_message failed".to_string()),
        }
    }

    pub fn as_channel_post(&self) -> Result<&Message, String> {
        match self {
            InboundData::ChannelPost(ref m) => Ok(m),
            _ => Err("as_channel_post failed".to_string()),
        }
    }

    pub fn as_edited_channel_post(&self) -> Result<&Message, String> {
        match self {
            InboundData::EditedChannelPost(ref m) => Ok(m),
            _ => Err("as_edited_channel_post failed".to_string()),
        }
    }

    pub fn as_my_chat_member(&self) -> Result<&ChatMemberUpdated, String> {
        match self {
            InboundData::MyChatMember(ref c) => Ok(c),
            _ => Err("as_my_chat_member failed".to_string()),
        }
    }

    pub fn as_chat_member(&self) -> Result<&ChatMemberUpdated, String> {
        match self {
            InboundData::ChatMember(ref c) => Ok(c),
            _ => Err("as_chat_member failed".to_string()),
        }
    }

    pub fn as_chat_join_request(&self) -> Result<&ChatJoinRequest, String> {
        match self {
            InboundData::ChatJoinRequest(ref c) => Ok(c),
            _ => Err("as_chat_join_request failed".to_string()),
        }
    }
}

pub mod outbound {
    use std::collections::HashMap;

    use serde::Serialize;
    use serde_json::{json, Value};

    #[derive(Serialize)]
    pub struct OutboundData {
        chat_id: String,

        #[serde(flatten)]
        extra: HashMap<String, Value>,
    }

    pub enum ChatId {
        /// Unique identifier for the target chat
        Id(i64),
        /// Username of the target chat (in the format `@channelusername`)
        Name(String),
    }

    impl From<i64> for ChatId {
        fn from(n: i64) -> Self {
            Self::Id(n)
        }
    }

    impl From<String> for ChatId {
        fn from(n: String) -> Self {
            Self::Name(n)
        }
    }

    impl ToString for ChatId {
        fn to_string(&self) -> String {
            match &*self {
                ChatId::Id(n) => n.to_string(),
                ChatId::Name(n) => n.clone(),
            }
        }
    }

    #[derive(Serialize)]
    pub enum ParseMode {
        Markdown,
        MarkdownV2,
        HTML,
    }

    /// Send a text message.
    ///
    /// eg.
    /// ```rust
    /// outbound::message(message.chat.id, "__PONG\\!__")
    ///     .reply(message.message_id)
    ///     .parse_mode(ParseMode::MarkdownV2)
    ///     .build()
    /// ```
    pub fn message<C: Into<ChatId>, T: Into<String>>(chat_id: C, message: T) -> OutboundData {
        OutboundData {
            chat_id: chat_id.into().to_string(),
            extra: [("text".to_string(), json!(message.into()))]
                .into_iter()
                .collect(),
        }
    }

    /// Send a text message.
    ///
    /// eg.
    /// ```rust
    /// outbound::edit_message(message.chat.id, message.message_id, "__PONG\\!__")
    ///     .parse_mode(ParseMode::MarkdownV2)
    ///     .build()
    /// ```
    pub fn edit_message<C: Into<ChatId>, M: Into<String>, T: Into<String>>(
        chat_id: C,
        message_id: M,
        message: T,
    ) -> OutboundData {
        OutboundData {
            chat_id: chat_id.into().to_string(),
            extra: [
                ("message_id".to_string(), json!(message_id.into())),
                ("text".to_string(), json!(message.into())),
            ]
            .into_iter()
            .collect(),
        }
    }

    /// Ban a user in a chat.
    ///
    /// eg.
    /// ```rust
    /// outbound::ban(message.chat.id, message.from.id)
    ///     .build()
    /// ```
    pub fn ban<C: Into<ChatId>, T: Into<String>>(chat_id: C, user_id: T) -> OutboundData {
        OutboundData {
            chat_id: chat_id.into().to_string(),
            extra: [("user_id".to_string(), json!(user_id.into()))]
                .into_iter()
                .collect(),
        }
    }

    impl OutboundData {
        /// Build outbound JSON data.
        pub fn build(self) -> Result<String, String> {
            if self.extra.is_empty() {
                return Err("OutboundData build failed: Too few fields".to_string());
            }

            serde_json::to_string(&self)
                .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
        }

        /// Reply to by original message ID.
        pub fn reply<M: Into<String>>(mut self, message_id: M) -> Self {
            self.extra
                .insert("reply_to_message_id".to_string(), json!(message_id.into()));
            self
        }

        /// Mode for parsing entities in the message text.
        pub fn parse_mode(mut self, mode: ParseMode) -> Self {
            self.extra.insert("parse_mode".to_string(), json!(mode));
            self
        }

        /// Add extra field.
        pub fn field<K: Into<String>>(mut self, name: K, value: Value) -> Self {
            self.extra.insert(name.into(), value);
            self
        }
    }
}
