use std::collections::BTreeSet;
use std::time::{Duration, SystemTime};

use url::Url;

use serde_json::json;

#[derive(Debug)]
pub enum Event {
    Ready(Ready), //0
    Heartbeat(Heartbeat), //1
    Identify(Identify), //2
    Hello(Hello), //10
    HeartbeatACK, //11
}

#[derive(Debug)]
pub struct Ready {
    v: u8,
    user: User,
    guilds: Vec<UnavailableGuild>,
    session_id: String,
    resume_gateway_url: Url,

}

#[derive(Debug)]
pub struct Heartbeat {
    last_sequence_number: Option<u64>,
}

#[derive(Debug)]
pub struct Identify {
    token: String,
    properties: ConnectionProperties,
    // compress: bool,
    large_threshold: u8,
    // shard: Option<(u64,u64)>,
    // presence: Option<PresenceUpdate>,
    // intents: BTreeSet<Intent>,
}

#[derive(Debug)]
pub struct Hello {
    pub heartbeat_interval: Duration,
}

#[derive(Debug)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    bot: bool,
}

#[derive(Debug)]
pub struct UnavailableGuild {
    id: String,
    unavailable: bool,
}

#[derive(Debug)]
pub struct ConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

// pub struct PresenceUpdate {
//     since: Option<SystemTime>,
//     activities: Vec<Activity>,
//     status: Status,
//     afk: bool,
// }

// pub enum Intent {
//     Guilds,
//     //(1 << 0)
//     //- GUILD_CREATE
//     //- GUILD_UPDATE
//     //- GUILD_DELETE
//     //- GUILD_ROLE_CREATE
//     //- GUILD_ROLE_UPDATE
//     //- GUILD_ROLE_DELETE
//     //- CHANNEL_CREATE
//     //- CHANNEL_UPDATE
//     //- CHANNEL_DELETE
//     //- CHANNEL_PINS_UPDATE
//     //- THREAD_CREATE
//     //- THREAD_UPDATE
//     //- THREAD_DELETE
//     //- THREAD_LIST_SYNC
//     //- THREAD_MEMBER_UPDATE
//     //- THREAD_MEMBERS_UPDATE *
//     //- STAGE_INSTANCE_CREATE
//     //- STAGE_INSTANCE_UPDATE
//     //- STAGE_INSTANCE_DELETE

//     GuildMembers,
//     //(1 << 1)
//     //- GUILD_MEMBER_ADD
//     //- GUILD_MEMBER_UPDATE
//     //- GUILD_MEMBER_REMOVE
//     //- THREAD_MEMBERS_UPDATE *

//     GuildModeration,
//     //(1 << 2)
//     //- GUILD_AUDIT_LOG_ENTRY_CREATE
//     //- GUILD_BAN_ADD
//     //- GUILD_BAN_REMOVE

//     GuildEmojisAndStickers,
//     //(1 << 3)
//     //- GUILD_EMOJIS_UPDATE
//     //- GUILD_STICKERS_UPDATE

//     GuildIntegration,
//     //(1 << 4)
//     //- GUILD_INTEGRATIONS_UPDATE
//     //- INTEGRATION_CREATE
//     //- INTEGRATION_UPDATE
//     //- INTEGRATION_DELETE

//     GuildWebhooks,
//     //(1 << 5)
//     //- WEBHOOKS_UPDATE

//     GuildInvites,
//     //(1 << 6)
//     //- INVITE_CREATE
//     //- INVITE_DELETE

//     GuildVoiceStates,
//     //(1 << 7)
//     //- VOICE_STATE_UPDATE

//     GuildPresences,
//     //(1 << 8) **
//     //- PRESENCE_UPDATE

//     GuildMessages,
//     //(1 << 9)
//     //- MESSAGE_CREATE
//     //- MESSAGE_UPDATE
//     //- MESSAGE_DELETE
//     //- MESSAGE_DELETE_BULK

//     GuildMessageReactions,
//     //(1 << 10)
//     //- MESSAGE_REACTION_ADD
//     //- MESSAGE_REACTION_REMOVE
//     //- MESSAGE_REACTION_REMOVE_ALL
//     //- MESSAGE_REACTION_REMOVE_EMOJI

//     GuildMessageTyping,
//     //(1 << 11)
//     //- TYPING_START

//     DirectMessages,
//     //(1 << 12)
//     //- MESSAGE_CREATE
//     //- MESSAGE_UPDATE
//     //- MESSAGE_DELETE
//     //- CHANNEL_PINS_UPDATE

//     DirectMessageReactions,
//     //(1 << 13)
//     //- MESSAGE_REACTION_ADD
//     //- MESSAGE_REACTION_REMOVE
//     //- MESSAGE_REACTION_REMOVE_ALL
//     //- MESSAGE_REACTION_REMOVE_EMOJI

//     DirectMessageTyping,
//     //(1 << 14)
//     //- TYPING_START

//     MessageContent,
//     //(1 << 15)

//     GuildScheduledEvent,
//     //(1 << 16)
//     //- GUILD_SCHEDULED_EVENT_CREATE
//     //- GUILD_SCHEDULED_EVENT_UPDATE
//     //- GUILD_SCHEDULED_EVENT_DELETE
//     //- GUILD_SCHEDULED_EVENT_USER_ADD
//     //- GUILD_SCHEDULED_EVENT_USER_REMOVE

//     AutoModerationConfiguration,
//     //(1 << 20)
//     //- AUTO_MODERATION_RULE_CREATE
//     //- AUTO_MODERATION_RULE_UPDATE
//     //- AUTO_MODERATION_RULE_DELETE

//     AutoModerationExecution,
//     //(1 << 21)
//     //- AUTO_MODERATION_ACTION_EXECUTION
// }

// pub type Activity = (); //TODO sounds tedious

// pub enum Status {
//     Online,
//     DoNotDisturb,
//     AFK,
//     Invisible,
//     Offline,
// }

#[derive(Debug)]
pub enum TryFromJSONError {
    NotAnObject,
    NoOpCode,
    WrongOpCodeKind,
    UnsoundHeartbeatInterval,
    UnsupportedOpCode(u64),
}

impl Event {
    pub fn hello(heartbeat_interval: Duration) -> Event {
        Event::Hello(Hello { heartbeat_interval })
    }

    pub fn heartbeat(last_sequence_number: Option<u64>) -> Event {
        Event::Heartbeat(Heartbeat {
            last_sequence_number,
        })
    }

    pub fn identify(token: String, properties: ConnectionProperties, large_threshold: u8) -> Event {
        Event::Identify(Identify { token, properties, large_threshold, })
    }

    pub fn to_json(self) -> serde_json::Value {
        <Event as Into<serde_json::Value>>::into(self)
    }

    pub fn to_ws_msg(self) -> tokio_tungstenite::tungstenite::Message {
        tokio_tungstenite::tungstenite::Message::Text(
            self.to_json().to_string(),
        )
    }
}

impl Into<serde_json::Value> for Event {
    fn into(self) -> serde_json::Value {
        match self {
            Event::Ready(rd) => json!(
                {
                    "op": 0,
                    "d": {},
                }
            ),
            Event::Hello(Hello { heartbeat_interval }) => json!(
                {
                    "op": 10,
                    "d": {
                        "heartbeat_interval": heartbeat_interval,
                    }
                }
            ),
            Event::Identify(id) => json!(
                {
                    "op": 2,
                    "d": {
                        "token": id.token,
                        "properties": <ConnectionProperties as Into<serde_json::Value>>::into(id.properties),
                        "large_threshold": id.large_threshold,
                    },
                }
            ),
            Event::Heartbeat(hb) => json!(
                {
                    "op": 1,
                    "d": hb.last_sequence_number,
                }
            ),
            Event::HeartbeatACK => json!(
                {
                    "op": 11,
                }
            ),
        }
    }
}

impl TryFrom<serde_json::Value> for Event {
    type Error = TryFromJSONError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        use serde_json::Value::*;
        use TryFromJSONError::*;

        match value {
            Object(map) => match map.get("op") {
                Some(Number(n)) => match n.as_u64() {
                    Some(0) => {
                        println!("{}", serde_json::to_string_pretty(&Object(map)).unwrap());
                        todo!()
                    },
                    Some(10) => map
                        .get("d")
                        .and_then(|m| m.get("heartbeat_interval"))
                        .and_then(|v| v.as_u64())
                        .map(|hb| Duration::from_millis(hb))
                        .map_or(Err(UnsoundHeartbeatInterval), |hb| Ok(Event::hello(hb))),
                    Some(11) => Ok(Event::HeartbeatACK),
                    Some(n) => Err(UnsupportedOpCode(n)),
                    None => Err(WrongOpCodeKind),
                },
                Some(_) => Err(WrongOpCodeKind),
                None => Err(NoOpCode),
            },
            _ => Err(NotAnObject),
        }
    }
}

impl Default for ConnectionProperties {
    fn default() -> Self {
        ConnectionProperties {
            os: "linux".to_string(),
            browser: "chrome".to_string(),
            device: "pc".to_string(),
        }
    }
}

impl Into<serde_json::Value> for ConnectionProperties {
    fn into(self) -> serde_json::Value {
        json!(
            {
                "os": self.os,
                "browser": self.browser,
                "device": self.device,
            }
        )
    }
}

impl std::fmt::Display for TryFromJSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TryFromJSONError::*;
        match self {
            NotAnObject => write!(f, "The received JSON value is not an object."),
            NoOpCode => write!(f, "No op code available to decode object."),
            WrongOpCodeKind => write!(f, "The op code is not a number."),
            UnsoundHeartbeatInterval => write!(f, "The received heartbeat is unsound"),
            UnsupportedOpCode(op) => write!(f, "The op code {} is not supported", op),
        }
    }
}

impl std::error::Error for TryFromJSONError {}
