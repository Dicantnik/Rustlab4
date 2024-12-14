use rocket::serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, Unexpected};


fn deserialize_str_or_number<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Number(n) if n.is_i64() => {
            n.as_i64().map(|v| v as i32).ok_or_else(|| de::Error::invalid_type(Unexpected::Other("i64"), &"i32"))
        }
        serde_json::Value::String(s) => s.parse::<i32>().map_err(de::Error::custom),
        _ => Err(de::Error::invalid_type(Unexpected::Other("string or number"), &"i32")),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewMessageRequest {
    #[serde(deserialize_with = "deserialize_str_or_number")]
    pub chat_id: i32,
    #[serde(deserialize_with = "deserialize_str_or_number")]
    pub user_id: i32,
    pub content: String,
    pub file_data: Option<String>,
    pub file_path: Option<String>,
    pub message_type: String,
}