use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone)]
pub enum TextEntity {
    Lemmatizable(String),
    Illemmatizable(String),
}

#[derive(Clone)]
pub struct Message {
    pub id: usize,
    pub text_entities: Vec<TextEntity>,
    pub date_unixtime: u32,
    pub reply_to_message_id: Option<usize>,
}

impl From<DeserializedMessage> for Message {
    fn from(value: DeserializedMessage) -> Self {
        let text_entities = value
            .text_entities
            .into_iter()
            .map(|entity| match entity.entity_type {
                DeserializedTextEntityType::Link
                | DeserializedTextEntityType::CustomEmoji
                | DeserializedTextEntityType::MentionName
                | DeserializedTextEntityType::BotCommand
                | DeserializedTextEntityType::Phone
                | DeserializedTextEntityType::Mention
                | DeserializedTextEntityType::Code
                | DeserializedTextEntityType::Email => TextEntity::Illemmatizable(entity.text),
                _ => TextEntity::Lemmatizable(entity.text),
            })
            .collect();
        Message {
            id: value.id as usize,
            text_entities,
            date_unixtime: value.date_unixtime,
            reply_to_message_id: value.reply_to_message_id.map(|id| id as usize),
        }
    }
}

impl From<TextEntity> for String {
    fn from(value: TextEntity) -> String {
        match value {
            TextEntity::Lemmatizable(text) => text,
            TextEntity::Illemmatizable(text) => text,
        }
    }
}

impl From<Message> for String {
    fn from(value: Message) -> String {
        value
            .text_entities
            .into_iter()
            .map(|entity| String::from(entity))
            .collect::<String>()
    }
}

pub fn deserialize_messages(json: &str) -> Result<Vec<Message>, anyhow::Error> {
    let chat: Chat = serde_json::from_str(json)?;
    let mut pruned_ids = HashMap::new();
    let mut current_id = 0;
    let mut messages = chat
        .messages
        .into_iter()
        .map(Message::from)
        .collect::<Vec<Message>>();
    for message in &mut messages {
        if let Some(reply_to_message_id) = message.reply_to_message_id {
            if let Some(pruned_reply_to_message_id) = pruned_ids.get(&reply_to_message_id) {
                message.reply_to_message_id = Some(*pruned_reply_to_message_id);
            }
        }
        pruned_ids.insert(message.id, current_id);
        message.id = current_id;
        current_id += 1;
    }
    Ok(messages)
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn filter_service_messages<'de, D>(deserializer: D) -> Result<Vec<DeserializedMessage>, D::Error>
where
    D: Deserializer<'de>,
{
    let messages: Vec<DeserializedMessage> = Deserialize::deserialize(deserializer)?;
    Ok(messages
        .into_iter()
        .filter(|message| message.message_type != DeserializedMessageType::Service)
        .collect())
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum DeserializedMessageType {
    Service,
    Message,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum DeserializedTextEntityType {
    Link,
    CustomEmoji,
    Spoiler,
    MentionName,
    Bold,
    TextLink,
    BotCommand,
    Pre,
    Plain,
    Phone,
    Underline,
    Strikethrough,
    Mention,
    Blockquote,
    Code,
    Italic,
    Hashtag,
    Email,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DeserializedTextEntity {
    #[serde(rename = "type")]
    entity_type: DeserializedTextEntityType,
    text: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DeserializedMessage {
    pub id: u32,
    #[serde(rename = "type")]
    pub message_type: DeserializedMessageType,
    #[serde(deserialize_with = "from_str")]
    pub date_unixtime: u32,
    pub text_entities: Vec<DeserializedTextEntity>,
    #[serde(default)]
    pub reply_to_message_id: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Chat {
    #[serde(deserialize_with = "filter_service_messages")]
    messages: Vec<DeserializedMessage>,
}

mod tests {
    use super::*;

    const JSON: &str = r#"
{
 "name": "Group Name",
 "type": "public_supergroup",
 "id": 123123123,
 "messages": [
  {
   "id": 1,
   "type": "service",
   "date": "2020-05-28T21:01:01",
   "date_unixtime": "1590691915",
   "actor": "Group Name",
   "actor_id": "channel123",
   "action": "create_channel",
   "title": "ABC123",
   "text": "",
   "text_entities": []
  },
  {
   "id": 3,
   "type": "message",
   "date": "2020-05-28T21:53:41",
   "date_unixtime": "1590692021",
   "from": "User1",
   "from_id": "channel1244897058",
   "forwarded_from": "User1",
   "saved_from": "User1",
   "photo": "(File not included. Change data exporting settings to download.)",
   "width": 720,
   "height": 338,
   "text": [
    {
     "type": "bold",
     "text": "abc"
    },
    "123",
    {
     "type": "link",
     "text": "https://google.com/"
    },
    ""
   ],
   "text_entities": [
    {
     "type": "bold",
     "text": "abc"
    },
    {
     "type": "plain",
     "text": "123"
    },
    {
     "type": "link",
     "text": "https://google.com/"
    },
    {
     "type": "plain",
     "text": ""
    }
   ]
  }
 ]
}
    "#;

    #[test]
    fn test_deserialize() {
        let chat: Chat = serde_json::from_str(JSON).unwrap();
        let m = DeserializedMessage {
            id: 3,
            message_type: DeserializedMessageType::Message,
            date_unixtime: 1590692021,
            text_entities: vec![
                DeserializedTextEntity {
                    entity_type: DeserializedTextEntityType::Bold,
                    text: "abc".to_string(),
                },
                DeserializedTextEntity {
                    entity_type: DeserializedTextEntityType::Plain,
                    text: "123".to_string(),
                },
                DeserializedTextEntity {
                    entity_type: DeserializedTextEntityType::Link,
                    text: "https://google.com/".to_string(),
                },
                DeserializedTextEntity {
                    entity_type: DeserializedTextEntityType::Plain,
                    text: "".to_string(),
                },
            ],
            reply_to_message_id: None,
        };
        let expected = Chat { messages: vec![m] };

        assert_eq!(chat, expected);
    }
}
