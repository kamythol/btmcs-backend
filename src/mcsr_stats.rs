use cached::proc_macro::cached;
use anyhow::{Error, Result};
use chrono_tz::{Tz, US::Pacific};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use chrono::{prelude::*, TimeZone};

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
    losses_today: u32,
    ffs_season: u32,
    ffs_today: u32,
    ff_wins_season: u32,
    ff_wins_today: u32,
    slowest_season: u32,
    slowest_today: u32,
    fastest_today: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Final {
    elo: u32,
    today: Today,
    season: Season,
    overall: Overall,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Today {
    matches: u32,
    deaths: u32,
    elo: i32,
    wins: u32,
    draws: u32,
    losses: u32,
    forfeits: u32,
    forfeit_wins: u32,
    slowest: String,
    fastest: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Season {
    matches: u32,
    deaths: u32,
    elo_peak: u32,
    elo_lowest: u32,
    pb: String,
    forfeits: u32,
    forfeit_wins: u32,
    slowest: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Overall {
    elo_peak: u32,
    elo_lowest: u32,
    pb: String,
}
const UUID: &str = "8a8174eb699a49fcb2299af5eede0992";

async fn get_match(match_id: u32, season: u8) -> Result<match_data::GameData, Error> {
    let req = format!("https://mcsrranked.com/api/matches/{}?season={}", match_id, season);
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<match_data::Response>().await?;
    Ok(data.data)
}

async fn get_history() -> Result<Vec<match_history::GameData>, Error> {
    // -- Season 9 -- //
    // let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4526605"); // 100
    // let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4424617"); // 160

    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4857369"); // 139
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<match_history::Response>().await?;
    Ok(data.data)
}

async fn get_profile() -> Result<profile_data::Data, Error> {
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc");
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<profile_data::Response>().await?;
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
    let tl_death = "projectelo.timeline.death".to_string();
    let tl_forfeit = "projectelo.timeline.forfeit".to_string();
    let current_pst: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
    
    let mh = get_history().await.expect("augh");
    // -- Season 9 -- //
    // let mut matches: u32 = 160; // match count offset - last: 100
    // let mut deaths: u32 = 135; // death count offset - last: 80

    // offsets break todays, only update at 0 utc
    let mut matches: u32 = 139; // offset
    let mut deaths: u32 = 87; // offset
    let mut ffs_season: u32 = 4; // offset
    let mut ff_wins_season: u32 = 12; // offset
    let mut slowest_season: u32 = 0;
    
    let mut matches_today: u32 = 0;
    let mut deaths_today: u32 = 0;
    let mut elo_today: i32 = 0;
    let mut wins_today: u32 = 0;
    let mut draws_today: u32 = 0;
    let mut ffs_today: u32 = 0;
    let mut ff_wins_today: u32 = 0;
    let mut slowest_today: u32 = 0;
    let mut fastest_today: u32 = 99999999;

    for m in mh {
        matches += 1;
        let t = Utc.timestamp_opt(m.date as i64, 0).unwrap().with_timezone(&Pacific);
        let gd = get_match(m.id, m.season).await.unwrap();
        if t.day() == current_pst.day() {
            matches_today += 1;
            for plr in gd.changes {
                if plr.uuid == UUID.to_string() { elo_today += plr.change.unwrap_or(0); }
            }
            if m.result.uuid.clone().unwrap_or("augh".to_string()) == UUID.to_string() { // win check
                wins_today += 1;
                if fastest_today > m.result.time && m.forfeited == false { fastest_today = m.result.time; }
                if slowest_today < m.result.time { slowest_today = m.result.time; }
                if m.forfeited == true { ff_wins_today += 1; }
            } else if m.result.uuid == Option::None {
                draws_today += 1;
            }
        }
        if slowest_season < m.result.time { slowest_season = m.result.time; }
        if (m.result.uuid.unwrap_or("a".to_string()) == UUID.to_string()) && (m.forfeited == true) {
            ff_wins_season += 1;
        }
        println!("Match {} S{} in {}m", m.id, m.season, m.result.time/1000/60);
        for timeline in gd.timelines {
            if (timeline.timeline_type == tl_death) && (timeline.uuid == UUID.to_string()) {
                if t.day() == current_pst.day() { deaths_today += 1; }
                deaths += 1;
            } else if timeline.timeline_type == tl_forfeit && (timeline.uuid == UUID.to_string()) {
                if t.day() == current_pst.day() { ffs_today += 1; }
                ffs_season += 1;
            } else { continue; }
        }
    } // this shit is so ass wilted flower emoji
    let losses_today = matches_today - wins_today - draws_today;
    return Counts {
        matches, 
        deaths, 
        matches_today, 
        deaths_today, 
        elo_today, 
        wins_today, 
        draws_today, 
        losses_today, 
        ffs_season, 
        ffs_today, 
        ff_wins_season, 
        ff_wins_today,
        slowest_season,
        slowest_today,
        fastest_today
    }
}

async fn get_overall_peaks() -> Vec<u32> { // peak elos
    let seasons = get_profile_seasons().await.expect("sea");
    let mut peak: u32 = 0;
    let mut lowest: u32 = 4000;
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
    let elo_today: i32 = counts.elo_today;
    let wins_today = counts.wins_today;
    let losses_today = counts.losses_today;
    let draws_today = counts.draws_today;
    let slowest_season_ms = counts.slowest_season;
    let slowest_today_ms = counts.slowest_today;
    let fastest_today_ms = counts.fastest_today;
    let season_best_ms = p.statistics.season.best_time.ranked.unwrap_or(0);
    let all_best_ms = p.statistics.total.best_time.ranked.unwrap_or(0);
    
    let season_best_fmt = format!("{}:{:02}", season_best_ms / 1000 / 60, season_best_ms / 1000 % 60);
    let all_best_fmt = format!("{}:{:02}", all_best_ms / 1000 / 60, all_best_ms / 1000 % 60);
    let slowest_season_fmt = format!("{}:{:02}", slowest_season_ms / 1000 / 60, slowest_season_ms / 1000 % 60);
    let slowest_today_fmt = format!("{}:{:02}", slowest_today_ms / 1000 / 60, slowest_today_ms / 1000 % 60);
    let fastest_today_fmt = format!("{}:{:02}", fastest_today_ms / 1000 / 60, fastest_today_ms / 1000 % 60);
    
    let a = Today {
        matches: counts.matches_today,
        deaths: counts.deaths_today,
        elo: elo_today,
        wins: wins_today,
        draws: draws_today,
        losses: losses_today,
        forfeits: counts.ffs_today,
        forfeit_wins: counts.ff_wins_today,
        slowest: slowest_today_fmt,
        fastest: fastest_today_fmt
    };
    let b = Season {
        matches,
        deaths,
        elo_peak: p.season_result.highest.unwrap_or(0),
        elo_lowest: p.season_result.lowest.unwrap_or(0),
        pb: season_best_fmt,
        forfeits: counts.ffs_season,
        forfeit_wins: counts.ff_wins_season,
        slowest: slowest_season_fmt
    };
    let c = Overall {
        elo_peak: get_overall_peaks().await[0],
        elo_lowest: get_overall_peaks().await[1],
        pb: all_best_fmt
    };
    let f = Final {
        elo: p.elo_rate.unwrap_or(0),
        today: a,
        season: b,
        overall: c
    };
    return Json(f)
}