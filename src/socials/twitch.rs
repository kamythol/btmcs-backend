use anyhow::Result;
use serde::{Deserialize};
use rocket::serde::{json::Json, Serialize};
use lazy_static::lazy_static;

mod config;

lazy_static!{
    static ref OAUTH_TOKEN: String = config::get_oauth();
    static ref CLIENTID: String = config::get_clientid();
}

#[derive(Serialize, Deserialize)]
struct ChannelInfo {
    data: Vec<Channel>,
    pagination: Pagination,
}
#[derive(Serialize, Deserialize)]
pub struct Channel {
    broadcaster_language: String,
    broadcaster_login: String,
    display_name: String,
    game_id: String,
    game_name: String,
    id: String,
    is_live: bool,
    tag_ids: Vec<String>,
    tags: Vec<String>,
    thumbnail_url: String,
    title: String,
    started_at: String,
}
#[derive(Serialize, Deserialize)]
struct StreamInfo {
    data: Vec<Stream>,
    pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct Stream {
    id: String,
    stream_id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    muted_segments: Option<Vec<Pagination>>,
}
#[derive(Serialize, Deserialize)]
struct Pagination {
}
#[derive(Serialize, Deserialize)]
struct Followers {
    total: u64,
    data: Vec<Pagination>,
    pagination: Pagination,
}

async fn get_twitch_info(channel: String) -> Result<Vec<Channel>, anyhow::Error> {
    let req = format!("https://api.twitch.tv/helix/search/channels?query={}&first=1", channel);
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .header("Authorization", format!("Bearer {}", OAUTH_TOKEN.as_str()))
        .header("Client-Id", CLIENTID.as_str())
        .send()
        .await?
        .json::<ChannelInfo>()
        .await?;

    Ok(data.data)
}

async fn get_followers() -> Result<u64, anyhow::Error> {
    let req = format!("https://api.twitch.tv/helix/channels/followers?broadcaster_id=46708418&first=1");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .header("Authorization", format!("Bearer {}", OAUTH_TOKEN.as_str()))
        .header("Client-Id", CLIENTID.as_str())
        .send()
        .await?
        .json::<Followers>()
        .await?;

    Ok(data.total)
}

async fn get_latest_streams() -> Result<Vec<Stream>, anyhow::Error> {
    let req = format!("https://api.twitch.tv/helix/videos?user_id=46708418&first=50");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .header("Authorization", format!("Bearer {}", OAUTH_TOKEN.as_str()))
        .header("Client-Id", CLIENTID.as_str())
        .send()
        .await?
        .json::<StreamInfo>()
        .await?;

    Ok(data.data)
}

#[get("/data")]
pub async fn twitchinfo() -> Json<Vec<Channel>> { // twitch channel info
    let info = get_twitch_info("btmc".to_string()).await.unwrap();
    return Json(info)
}

#[get("/followers")]
pub async fn followers() -> String { // follower count
    let followers = get_followers().await.unwrap().to_string();
    return followers
}

#[get("/latest")]
pub async fn latest_streams() -> Json<Vec<Stream>> { // latest stream info
    let streaminfo = get_latest_streams().await.unwrap();
    return Json(streaminfo)
}