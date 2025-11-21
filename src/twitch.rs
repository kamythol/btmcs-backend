use anyhow::Result;
use serde::{Deserialize};
use rocket::serde::{Serialize};

const OAUTH_TOKEN: &str = "";
const CLIENTID: &str = "";

#[derive(Serialize, Deserialize)]
pub struct ChannelInfo {
    pub(crate) data: Vec<Channel>,
    pagination: Pagination,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Channel {
    pub(crate) broadcaster_language: String,
    pub(crate) broadcaster_login: String,
    pub(crate) display_name: String,
    pub(crate) game_id: String,
    pub(crate) game_name: String,
    pub(crate) id: String,
    pub(crate) is_live: bool,
    pub(crate) tag_ids: Vec<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) thumbnail_url: String,
    pub(crate) title: String,
    pub(crate) started_at: String,
}
#[derive(Serialize, Deserialize)]
pub struct StreamInfo {
    pub(crate) data: Vec<Stream>,
    pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct Stream {
    id: String,
    stream_id: String,
    user_id: String,
    user_login: String,
    user_name: String,
    title: String,
    description: String,
    created_at: String,
    published_at: String,
    url: String,
    thumbnail_url: String,
    viewable: String,
    view_count: u64,
    language: String,
    #[serde[rename = "type"]]
    stream_type: String,
    duration: String,
    muted_segments: Vec<Pagination>,
}
#[derive(Serialize, Deserialize)]
pub struct Pagination {
}
#[derive(Serialize, Deserialize)]
pub struct Followers {
    pub(crate) total: u64,
    data: Vec<Pagination>,
    pagination: Pagination,
}

pub async fn is_live(channel: String) -> Result<Vec<Channel>, anyhow::Error> {
    let req = format!("https://api.twitch.tv/helix/search/channels?query={}&first=1", channel);
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .header("Authorization", format!("Bearer {OAUTH_TOKEN}"))
        .header("Client-Id", CLIENTID)
        .send()
        .await?
        .json::<ChannelInfo>()
        .await?;

    Ok(data.data)
}

pub async fn followers() -> Result<u64, anyhow::Error> {
    let req = format!("https://api.twitch.tv/helix/channels/followers?broadcaster_id=46708418&first=1");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .header("Authorization", format!("Bearer {OAUTH_TOKEN}"))
        .header("Client-Id", CLIENTID)
        .send()
        .await?
        .json::<Followers>()
        .await?;

    Ok(data.total)
}

pub async fn latest() -> Result<Vec<Stream>, anyhow::Error> {
    let req = format!("https://api.twitch.tv/helix/videos?user_id=46708418&first=1");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .header("Authorization", format!("Bearer {OAUTH_TOKEN}"))
        .header("Client-Id", CLIENTID)
        .send()
        .await?
        .json::<StreamInfo>()
        .await?;

    Ok(data.data)
}
