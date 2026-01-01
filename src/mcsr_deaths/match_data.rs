use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    status: String,
    pub(crate) data: GameData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    id: u32,
    #[serde(rename = "type")]
    match_type: u32, 
    seed: Option<Seed>,
    category: Option<String>,
    game_mode: String, 
    players: Vec<Player>,
    spectators: Vec<String>, 
    result: ResultData,
    forfeited: bool,
    decayed: bool,
    rank: Rank,
    vod: Vec<Vod>,
    pub(crate) changes: Vec<Change>,
    completions: Vec<Completions>, 
    pub(crate) timelines: Vec<Timeline>,
    beginner: bool,
    bot_source: Option<u32>,
    season: u32,
    pub(crate) date: u64,
    seed_type: String,
    bastion_type: String,
    tag: Option<String>,
    replay_exist: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Seed {
    id: Option<String>,
    overworld: Option<String>,
    nether: Option<String>,
    end_towers: Vec<i32>,
    variations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    uuid: String,
    nickname: String,
    role_type: u32,
    elo_rate: Option<u32>,
    elo_rank: Option<u32>,
    country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultData {
    uuid: Option<String>,
    time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    season: Option<u32>,
    all_time: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vod {
    uuid: String,
    url: String,
    starts_at: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub(crate) uuid: String,
    pub(crate) change: Option<i32>,
    elo_rate: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completions {
    uuid: String,
    time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeline {
    pub(crate) uuid: String,
    time: u32,
    #[serde(rename = "type")]
    pub(crate) timeline_type: String,
}
