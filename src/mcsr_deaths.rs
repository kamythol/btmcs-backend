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
pub struct Final {
    deaths: u32,
    matches: u32,
    deaths_today: u32,
    matches_today: u32,
    elo: u32,
    elo_today: i32,
    elo_peak_season: u32,
    elo_peak_overall: u32,
    season_best: String,
    all_best: String,
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

    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4703219");
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
pub async fn get_counts() -> Vec<u32> {
    let matchtext = "projectelo.timeline.death".to_string();
    let current_utc: DateTime<Utc> = Utc::now();
    let mh = get_history().await.expect("augh");
    // -- Season 9 -- //
    // let mut matches: u32 = 160; // match count offset - last: 100
    // let mut deaths: u32 = 135; // death count offset - last: 80

    let mut matches: u32 = 35; // offset
    let mut deaths: u32 = 24; // offset
    let mut matches_today: u32 = 0;
    let mut deaths_today: u32 = 0;
    let mut elo_today: i32 = 0;
    
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
    return vec![matches, deaths, matches_today, deaths_today, elo_today.try_into().unwrap()]
}

async fn get_overall_peak() -> u32 {
    let seasons = get_profile_seasons().await.expect("sea");
    let mut peak: u32 = 0;
    for season in seasons.season_results.values() {
        if season.highest > peak { peak = season.highest; }
    }
    return peak
}


#[cached(time = 120, sync_writes = "default")]
#[get("/deaths")]
pub async fn deaths() -> String{
    let counts = get_counts().await;
    let matches = counts[0];
    let deaths = counts[1];

    let f = format!("{} deaths in {} matches", deaths, matches);
    return f
}

#[cached(time = 300, sync_writes = "default")]
#[get("/data")]
pub async fn create_data() -> Json<Final>{
    let p = get_profile().await.expect("au");
    let counts = get_counts().await;
    let matches = counts[0];
    let deaths = counts[1];
    let matches_today = counts[2];
    let deaths_today = counts[3];
    let elo_today: i32 = counts[4].try_into().unwrap();

    let season_best_ms = p.statistics.season.best_time.ranked;
    let all_best_ms = p.statistics.total.best_time.ranked;
    let season_formatted = format!("{}:{:02}", season_best_ms.unwrap_or(0) / 1000 / 60, season_best_ms.unwrap_or(0) / 1000 % 60);
    let all_formatted = format!("{}:{:02}", all_best_ms.unwrap_or(0) / 1000 / 60, all_best_ms.unwrap_or(0) / 1000 % 60);
    let a = Final {
        deaths, 
        matches, 
        deaths_today, 
        matches_today, 
        elo: p.elo_rate.unwrap_or(0), 
        elo_peak_season: p.season_result.highest.unwrap_or(0),
        elo_peak_overall: get_overall_peak().await,
        elo_today,
        season_best: season_formatted, 
        all_best: all_formatted
    };
    return Json(a)
}