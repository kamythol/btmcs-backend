use cached::proc_macro::cached;
use anyhow::{Error, Result};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tokio::time::{Instant, Duration};
use chrono::prelude::*;

mod match_data;
mod match_history;
mod profile_data;
mod seasons_data;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Counts {
    matches: u32,
    deaths: u32,
    matches_today: u32,
    deaths_today: u32,
    elo_today: i32,
    wins_today: u32,
    draws_today: u32,
    losses_today: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Final {
    deaths: u32,
    matches: u32,
    deaths_today: u32,
    matches_today: u32,
    elo: u32,
    elo_today: i32,
    elo_peak_season: u32,
    elo_lowest_season: u32,
    elo_peak_overall: u32,
    elo_lowest_overall: u32,
    season_best: String,
    all_best: String,
    wins_today: u32,
    draws_today: u32,
    losses_today: u32
}
const UUID: &str = "8a8174eb699a49fcb2299af5eede0992";

async fn get_match(match_id: u32, season: u8) -> Result<match_data::GameData, Error> {
    let req = format!("https://mcsrranked.com/api/matches/{}?season={}", match_id, season);
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .send()
        .await?
        .json::<match_data::Response>()
        .await?;
    Ok(data.data)
}

async fn get_history() -> Result<Vec<match_history::GameData>, Error> {
    // -- Season 9 -- //
    // let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4526605"); // 100
    // let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4424617"); // 160

    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4732008"); // 57
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .send()
        .await?
        .json::<match_history::Response>()
        .await?;
    Ok(data.data)
}

async fn get_profile() -> Result<profile_data::Data, Error> {
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .send()
        .await?
        .json::<profile_data::Response>()
        .await?;
    Ok(data.data)
}

async fn get_profile_seasons() -> Result<seasons_data::Data, Error> {
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/seasons");
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<seasons_data::Response>().await?;
    Ok(data.data)
}

#[cached(time = 120, sync_writes = "default")]
pub async fn get_counts() -> Counts {
    let matchtext = "projectelo.timeline.death".to_string();
    let current_utc: DateTime<Utc> = Utc::now();
    let mh = get_history().await.expect("augh");
    // -- Season 9 -- //
    // let mut matches: u32 = 160; // match count offset - last: 100
    // let mut deaths: u32 = 135; // death count offset - last: 80

    let mut matches: u32 = 57; // offset
    let mut deaths: u32 = 34; // offset
    let mut matches_today: u32 = 0;
    let mut deaths_today: u32 = 0;
    let mut elo_today: i32 = 0;
    let mut wins_today: u32 = 0;
    let mut draws_today: u32 = 0;
    
    for m in mh {
        matches += 1;
        let t = Utc.timestamp_opt(m.date as i64, 0).unwrap();
        let gd = get_match(m.id, m.season).await.unwrap();
        if t.day() == current_utc.day() {
            matches_today += 1;
            for p in gd.changes {
                if p.uuid == UUID.to_string() {
                    elo_today += p.change.unwrap_or(0);
                }
            }
            if m.result.uuid.clone().unwrap_or("augh".to_string()) == UUID.to_string() {
                wins_today += 1;
            } else if m.result.uuid == Option::None {
                draws_today += 1;
            }
        }
        println!("Match {} S{} in {}m", m.id, m.season, m.result.time/1000/60);
        for timeline in gd.timelines {
            if (timeline.timeline_type == matchtext) &&
            (timeline.uuid == UUID.to_string()) {
                // println!("{:?}", timeline);
                if t.day() == current_utc.day() {
                    deaths_today += 1;
                }
                deaths += 1;
            } else {
                continue;
            }
        }
    }
    let losses_today = matches_today - wins_today;
    return Counts {matches, deaths, matches_today, deaths_today, elo_today, wins_today, draws_today, losses_today}
}

async fn get_overall_peaks() -> Vec<u32> {
    let seasons = get_profile_seasons().await.expect("sea");
    let mut peak: u32 = 0;
    let mut lowest: u32 = 0;
    for season in seasons.season_results.values() {
        if season.highest > peak { peak = season.highest; }
        if season.lowest < lowest { lowest = season.lowest; }
    }
    return vec![peak, lowest]
}


#[cached(time = 120, sync_writes = "default")]
#[get("/deaths")]
pub async fn deaths() -> String{
    let counts = get_counts().await;
    let matches = counts.matches;
    let deaths = counts.deaths;

    let f = format!("{} deaths in {} matches", deaths, matches);
    return f
}

#[cached(time = 300, sync_writes = "default")]
#[get("/data")]
pub async fn create_data() -> Json<Final>{
    let p = get_profile().await.expect("au");
    let counts = get_counts().await;
    let matches = counts.matches;
    let deaths = counts.deaths;
    let matches_today = counts.matches_today;
    let deaths_today = counts.deaths_today;
    let elo_today: i32 = counts.elo_today;
    let wins_today = counts.wins_today;
    let losses_today = counts.losses_today;
    let draws_today = counts.draws_today;
    // i have a headache rn i acn't figureout how to writ ethi sbetter
    let season_best_ms = p.statistics.season.best_time.ranked.unwrap_or(0);
    let all_best_ms = p.statistics.total.best_time.ranked.unwrap_or(0);
    let season_formatted = format!("{}:{:02}", season_best_ms / 1000 / 60, season_best_ms / 1000 % 60);
    let all_formatted = format!("{}:{:02}", all_best_ms / 1000 / 60, all_best_ms / 1000 % 60);
    let a = Final {
        deaths, 
        matches, 
        deaths_today, 
        matches_today, 
        elo: p.elo_rate.unwrap_or(0), 
        elo_peak_season: p.season_result.highest.unwrap_or(0),
        elo_lowest_season: p.season_result.lowest.unwrap_or(0),
        elo_peak_overall: get_overall_peaks().await[0],
        elo_lowest_overall: get_overall_peaks().await[1],
        elo_today,
        season_best: season_formatted, 
        all_best: all_formatted,
        wins_today,
        losses_today,
        draws_today
    };
    return Json(a)
}