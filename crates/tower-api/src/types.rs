use serde::{self, Serialize, Deserialize, Deserializer};
pub use chrono::{DateTime, Utc};

pub use config::{
    User,
    Token,
    Session,
};

#[derive(Serialize, Deserialize)]
pub struct App{
    pub name: String,
    pub short_description: String,
    pub owner: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Run {
    pub number: i32,
    pub app_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct AppSummary {
    pub app: App,

    #[serde(deserialize_with="parse_nullable_sequence")]
    pub runs: Vec<Run>,
}

#[derive(Serialize, Deserialize)]
pub struct Secret {
    pub name: String,
    pub preview: String,
    pub created_at: DateTime<Utc>,
}

/// parse_nullable_sequence is a helper function that deserializes a sequence of items that may be
/// null in the underlying data. This is useful for parsing content coming from the API that may or
/// may not be null if the resultant data is empty.
fn parse_nullable_sequence<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(Vec::new))
}
