use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Information about a user.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// The unique Id of the user. Can be used to calculate the account's
    /// creation date.
    pub id: String,
    /// Optional avatar hash.
    pub avatar: Option<String>,
    /// Indicator of whether the user is a bot.
    #[serde(default)]
    pub bot: bool,
    /// The account's discriminator to differentiate the user from others with
    /// the same [`Self::name`]. The name+discriminator pair is always unique.
    pub discriminator: String,
    /// The account's username. Changing username will trigger a discriminator
    /// change if the username+discriminator pair becomes non-unique.
    pub username: String,
    /// Optional banner hash.
    ///
    /// **Note**: This will only be present if the user is fetched via Rest API,
    /// e.g. with [`Http::get_user`].
    pub banner: Option<String>,
    /// The user's banner color encoded as an integer representation of
    /// hexadecimal color code
    ///
    /// **Note**: This will only be present if the user is fetched via Rest API,
    /// e.g. with [`Http::get_user`].
    pub accent_color: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    /// The unique ID given to this attachment.
    pub id: String,
    /// The filename of the file that was uploaded. This is equivalent to what
    /// the uploader had their file named.
    pub filename: String,
    /// If the attachment is an image, then the height of the image is provided.
    pub height: Option<u64>,
    /// The proxy URL.
    pub proxy_url: String,
    /// The size of the file in bytes.
    pub size: u64,
    /// The URL of the uploaded attachment.
    pub url: String,
    /// If the attachment is an image, then the width of the image is provided.
    pub width: Option<u64>,
    /// The attachment's [media type].
    ///
    /// [media type]: https://en.wikipedia.org/wiki/Media_type
    pub content_type: Option<String>,
    /// Whether this attachment is ephemeral.
    ///
    /// Ephemeral attachments will automatically be removed after a set period of time.
    ///
    /// Ephemeral attachments on messages are guaranteed to be available as long as
    /// the message itself exists.
    #[serde(default)]
    pub ephemeral: bool,
}

/// Differentiates between regular and different types of system messages.
#[derive(Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    /// A regular message.
    Regular = 0,
    /// An indicator that a recipient was added by the author.
    GroupRecipientAddition = 1,
    /// An indicator that a recipient was removed by the author.
    GroupRecipientRemoval = 2,
    /// An indicator that a call was started by the author.
    GroupCallCreation = 3,
    /// An indicator that the group name was modified by the author.
    GroupNameUpdate = 4,
    /// An indicator that the group icon was modified by the author.
    GroupIconUpdate = 5,
    /// An indicator that a message was pinned by the author.
    PinsAdd = 6,
    /// An indicator that a member joined the guild.
    MemberJoin = 7,
    /// An indicator that someone has boosted the guild.
    NitroBoost = 8,
    /// An indicator that the guild has reached nitro tier 1
    NitroTier1 = 9,
    /// An indicator that the guild has reached nitro tier 2
    NitroTier2 = 10,
    /// An indicator that the guild has reached nitro tier 3
    NitroTier3 = 11,
    /// An indicator that the channel is now following a news channel
    ChannelFollowAdd = 12,
    /// An indicator that the guild is disqualified for Discovery Feature
    GuildDiscoveryDisqualified = 14,
    /// An indicator that the guild is requalified for Discovery Feature
    GuildDiscoveryRequalified = 15,
    /// The first warning before guild discovery removal.
    GuildDiscoveryGracePeriodInitialWarning = 16,
    /// The last warning before guild discovery removal.
    GuildDiscoveryGracePeriodFinalWarning = 17,
    /// Message sent to inform users that a thread was created.
    ThreadCreated = 18,
    /// A message reply.
    InlineReply = 19,
    /// A slash command.
    ChatInputCommand = 20,
    /// A thread start message.
    ThreadStarterMessage = 21,
    /// Server setup tips.
    GuildInviteReminder = 22,
    /// A context menu command.
    ContextMenuCommand = 23,
    /// A message from an auto moderation action.
    AutoModerationAction = 24,
    /// An indicator that the message is of unknown type.
    Unknown = !0,
}

/// A partial amount of data for a member.
///
/// This is used in [`Message`]s from [`Guild`]s.
#[derive(Debug, Deserialize, Serialize)]
pub struct PartialMember {
    /// Indicator of whether the member can hear in voice channels.
    #[serde(default)]
    pub deaf: bool,
    /// Timestamp representing the date when the member joined.
    pub joined_at: Option<String>,
    /// Indicator of whether the member can speak in voice channels
    #[serde(default)]
    pub mute: bool,
    /// The member's nickname, if present.
    ///
    /// Can't be longer than 32 characters.
    pub nick: Option<String>,
    /// Vector of Ids of [`Role`]s given to the member.
    pub roles: Vec<String>,
    /// Indicator that the member hasn't accepted the rules of the guild yet.
    #[serde(default)]
    pub pending: bool,
    /// Timestamp representing the date since the member is boosting the guild.
    pub premium_since: Option<String>,
    /// The unique Id of the guild that the member is a part of.
    pub guild_id: Option<String>,
    /// Attached User struct.
    pub user: Option<User>,
}

/// Reference data sent with crossposted messages.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReference {
    /// ID of the originating message.
    pub message_id: Option<String>,
    /// ID of the originating message's channel.
    pub channel_id: String,
    /// ID of the originating message's guild.
    pub guild_id: Option<String>,
}

/// A representation of a message over a guild's text channel, a group, or a
/// private channel.
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    /// The unique Id of the message. Can be used to calculate the creation date
    /// of the message.
    pub id: String,
    /// An vector of the files attached to a message.
    pub attachments: Vec<Attachment>,
    /// The user that sent the message.
    pub author: User,
    /// The Id of the [`Channel`] that the message was sent to.
    pub channel_id: String,
    /// The content of the message.
    pub content: String,
    /// The timestamp of the last time the message was updated, if it was.
    pub edited_timestamp: Option<String>,
    /// The Id of the [`Guild`] that the message was sent in. This value will
    /// only be present if this message was received over the gateway.
    pub guild_id: Option<String>,
    /// Indicator of the type of message this is, i.e. whether it is a regular
    /// message or a system message.
    #[serde(rename = "type")]
    pub kind: MessageType,
    /// A partial amount of data about the user's member data, if this message
    /// was sent in a guild.
    pub member: Option<PartialMember>,
    /// Indicator of whether the message mentions everyone.
    pub mention_everyone: bool,
    /// Array of [`Role`]s' Ids mentioned in the message.
    pub mention_roles: Vec<String>,
    /// Array of users mentioned in the message.
    pub mentions: Vec<User>,
    /// Non-repeating number used for ensuring message order.
    #[serde(default)]
    pub nonce: Value,
    /// Indicator of whether the message is pinned.
    pub pinned: bool,
    /// Initial message creation timestamp, calculated from its Id.
    pub timestamp: String,
    /// Indicator of whether the command is to be played back via
    /// text-to-speech.
    ///
    /// In the client, this is done via the `/tts` slash command.
    pub tts: bool,
    /// Reference data sent with crossposted messages.
    pub message_reference: Option<MessageReference>,
    /// Array of message sticker item objects.
    /// The message that was replied to using this message.
    pub referenced_message: Option<Box<Message>>, // Boxed to avoid recursion
}

pub type InboundData = Message;

/// Deserialize the Discord Webhook Payload into `InboundData`.
pub fn inbound(s: String) -> Result<InboundData, String> {
    #[cfg(debug_assertions)]
    return serde_json::from_str::<InboundData>(&s)
        .map_err(|e| format!("Parsing Discord Webhook payload failed: {}", e.to_string()));

    #[cfg(not(debug_assertions))]
    serde_json::from_str::<InboundData>(&s)
        .map_err(|_| format!("Parsing Discord Webhook payload failed: {}", s))
}

pub mod outbound {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct OutboundData {
        content: String,
        reply_to: Option<super::InboundData>,
    }

    impl OutboundData {
        /// Build outbound JSON data.
        pub fn build(self) -> Result<String, String> {
            serde_json::to_string(&self)
                .map_err(|e| format!("OutboundData build failed: {}", e.to_string()))
        }
    }

    /// Send or reply to a message.
    /// 
    /// eg.
    /// ```rust
    /// outbound::say("hello world")
    ///     .build()
    /// ```
    pub fn say<C: Into<String>>(content: C, reply_to: Option<super::InboundData>) -> OutboundData {
        OutboundData {
            content: content.into(),
            reply_to,
        }
    }
}
