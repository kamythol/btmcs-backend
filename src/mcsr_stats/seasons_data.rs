use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    status: String,
    pub(crate) data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    uuid: String,
    nickname: String,
    role_type: u32,
    elo_rate: u32,
    elo_rank: u32,
    country: Option<String>,
    pub(crate) season_results: HashMap<String, SeasonData>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SeasonResults {
//     #[serde(rename = "7")]
//     season_7: SeasonData,
//     #[serde(rename = "8")]
//     season_8: SeasonData,
//     #[serde(rename = "9")]
//     season_9: SeasonData,
//     #[serde(rename = "10")]
//     season_10: SeasonData,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonData { // elo values
    last: LastResult,
    pub(crate) highest: u32,
    pub(crate) lowest: u32,
    phases: Vec<Phase>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastResult {
    elo_rate: u32,
    elo_rank: Option<u32>,
    phase_point: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase {
}
