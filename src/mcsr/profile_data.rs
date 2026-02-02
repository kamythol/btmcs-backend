use serde::{Deserialize, Serialize};

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
    pub(crate) elo_rate: Option<u32>,
    pub(crate) elo_rank: Option<u32>,
    achievements: Achievements,
    timestamp: Timestamp,
    pub(crate) statistics: Statistics,
    connections: Connections,
    weekly_races: Vec<WeeklyRaces>,
    pub(crate) season_result: SeasonResult,
    country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievements {
    display: Vec<Achievement>,
    total: Vec<Achievement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievement {
    id: String,
    date: u64,
    data: Vec<String>,
    level: u32,
    value: Option<u32>,
    goal: Option<u32>, 
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    next_decay: Option<u64>,
    first_online: u64,
    last_online: u64,
    last_ranked: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub(crate) season: SeasonStatistics,
    pub(crate) total: TotalStatistics,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeasonStatistics {
    pub(crate) best_time: Ranking,
    highest_win_streak: WinStreak,
    current_win_streak: WinStreak,
    played_matches: Matches,
    playtime: Playtime,
    completion_time: CompletionTime,
    forfeits: Forfeits,
    completions: Completions,
    pub(crate) wins: Matches,
    pub(crate) loses: Matches,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TotalStatistics {
    pub(crate) best_time: Ranking,
    highest_win_streak: WinStreak,
    current_win_streak: WinStreak,
    played_matches: Matches,
    playtime: Playtime,
    completion_time: CompletionTime,
    forfeits: Forfeits,
    completions: Completions,
    wins: Matches,
    loses: Matches,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ranking {
    pub(crate) ranked: Option<u32>,
    casual: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WinStreak {
    ranked: u32,
    casual: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Matches {
    pub(crate) ranked: u32,
    casual: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Playtime {
    ranked: u64,
    casual: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionTime {
    ranked: u64,
    casual: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forfeits {
    ranked: u32,
    casual: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completions {
    ranked: u32,
    casual: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connections {
    id: Option<String>,
    name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeeklyRaces {
    id: Option<u32>,
    time: Option<u32>,
    rank: Option<u32>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonResult {
    last: LastSeason,
    pub(crate) highest: Option<u32>,
    pub(crate) lowest: Option<u32>,
    phases: Option<Vec<Phases>>, 
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastSeason {
    elo_rate: Option<u32>,
    elo_rank: Option<u32>,
    phase_point: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Phases {
    phase: u32,
    elo_rate: u32,
    elo_rank: u32,
    point: u32
}