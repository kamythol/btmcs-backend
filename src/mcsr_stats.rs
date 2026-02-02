use cached::proc_macro::cached;
use chrono_tz::{Tz, US::Pacific};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use chrono::{prelude::*, TimeZone};

use crate::mcsr::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
    fastest_today: u32,
    resets_season: u32,
    resets_today: u32,
    avg_today: u32,
    death_wins: u32,
    deathless_wins: u32
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
    fastest: String,
    resets: u32,
    avg: String
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
    resets: u32,
    death_wins: u32,
    deathless_wins: u32,
    wins: u32,
    losses: u32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Overall {
    elo_peak: u32,
    elo_lowest: u32,
    pb: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Session {
    matches: u32,
    deaths: u32,
    elo: i32,
    wins: u32,
    draws: u32,
    losses: u32,
    forfeits: u32,
    forfeit_wins: u32,
    slowest: String,
    fastest: String,
    avg: String,
    resets: u32,
}

const UUID: &str = "8a8174eb699a49fcb2299af5eede0992";
// offsets break todays, only update at 0 pst
const MATCH_OFFSET: u32 = 544;
const DEATH_OFFSET: u32 = 366;
const FFS_SEASON_OFFSET: u32 = 12;
const FF_WINS_SEASON_OFFSET: u32 = 46;
const RESETS_SEASON_OFFSET: u32 = 226;



#[cached(time = 120, sync_writes = "default")]
pub async fn get_counts() -> Counts {
    let current_pst: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
    let uuid = UUID.to_string();
    let mh = get_history().await.expect("augh");
    // -- Season 9 -- //
    // let mut matches: u32 = 160; // match count offset - last: 100
    // let mut deaths: u32 = 135; // death count offset - last: 80

    let slowest_season: u32 = get_slowest().await.expect("a").first().unwrap().result.time;
    
    let mut b = Counts {
        matches: MATCH_OFFSET,
        deaths: DEATH_OFFSET,
        ffs_season: FFS_SEASON_OFFSET,
        ff_wins_season: FF_WINS_SEASON_OFFSET,
        resets_season: RESETS_SEASON_OFFSET,
        slowest_season,
        fastest_today: 99999999,
        ..Default::default()
    };
    for m in mh {
        b.matches += 1;
        let t = Utc.timestamp_opt(m.date as i64, 0).unwrap().with_timezone(&Pacific);
        let gd = get_match(m.id, m.season).await.unwrap();
        let is_win = m.result.uuid.clone().unwrap_or_else(|| "augh".to_string()) == uuid;

        if t.day() == current_pst.day() {
            b.matches_today += 1;
            for plr in gd.changes {
                if plr.uuid == uuid { b.elo_today += plr.change.unwrap_or(0); }
            }
            if is_win { // win check
                b.wins_today += 1;
                if !m.forfeited {
                    b.avg_today += m.result.time;
                    if b.fastest_today > m.result.time { b.fastest_today = m.result.time; }
                    if b.slowest_today < m.result.time { b.slowest_today = m.result.time; }
                } else {
                    b.ff_wins_today += 1; 
                }
            } else if m.result.uuid == Option::None {
                b.draws_today += 1;
            }
        }
        if (is_win) && (m.forfeited == true) {
            b.ff_wins_season += 1;
        }
        println!("Match {} S{} in {}m", m.id, m.season, m.result.time/1000/60);
        let mut has_death = false;
        for timeline in gd.timelines {
            if timeline.uuid != uuid { continue; }
            match timeline.timeline_type.as_str() {
                "projectelo.timeline.reset" => {
                    if t.day() == current_pst.day() { b.resets_today += 1; }
                    b.resets_season += 1;
                }
                "projectelo.timeline.death" => {
                    has_death = true;
                    if t.day() == current_pst.day() { b.deaths_today += 1; }
                    b.deaths += 1;
                }
                "projectelo.timeline.forfeit" => {
                    if t.day() == current_pst.day() { b.ffs_today += 1; }
                    b.ffs_season += 1;
                }
                _ => { }
            }
        }
        if is_win {
            if has_death {
                b.death_wins += 1;
            } else {
                b.deathless_wins += 1;
            }
        }
    } // this shit is so ass wilted flower emoji
    b.losses_today = b.matches_today - b.wins_today - b.draws_today;
    if b.wins_today == 0 || b.ff_wins_today == b.wins_today {
        b.avg_today = b.avg_today / 1;
    } else {
        b.avg_today = b.avg_today / (b.wins_today - b.ff_wins_today);
    }
    return Counts {
        matches: b.matches, 
        deaths: b.deaths, 
        matches_today: b.matches_today, 
        deaths_today: b.deaths_today, 
        elo_today: b.elo_today, 
        wins_today: b.wins_today, 
        draws_today: b.draws_today, 
        losses_today: b.losses_today, 
        ffs_season: b.ffs_season, 
        ffs_today: b.ffs_today, 
        ff_wins_season: b.ff_wins_season, 
        ff_wins_today: b.ff_wins_today,
        slowest_season: b.slowest_season,
        slowest_today: b.slowest_today,
        fastest_today: b.fastest_today,
        resets_season: b.resets_season,
        resets_today: b.resets_today,
        avg_today: b.avg_today,
        death_wins: b.death_wins,
        deathless_wins: b.deathless_wins
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
    let avg_today_ms = counts.avg_today;
    
    let season_best_fmt = format!("{}:{:02}", season_best_ms / 1000 / 60, season_best_ms / 1000 % 60);
    let all_best_fmt = format!("{}:{:02}", all_best_ms / 1000 / 60, all_best_ms / 1000 % 60);
    let slowest_season_fmt = format!("{}:{:02}", slowest_season_ms / 1000 / 60, slowest_season_ms / 1000 % 60);
    let slowest_today_fmt = format!("{}:{:02}", slowest_today_ms / 1000 / 60, slowest_today_ms / 1000 % 60);
    let avg_today_fmt = format!("{}:{:02}", avg_today_ms / 1000 / 60, avg_today_ms / 1000 % 60);
    let fastest_today_fmt: String;
    if fastest_today_ms == 99999999 {
        fastest_today_fmt = format!("6969:69");
    } else {
        fastest_today_fmt = format!("{}:{:02}", fastest_today_ms / 1000 / 60, fastest_today_ms / 1000 % 60);
    }
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
        fastest: fastest_today_fmt,
        resets: counts.resets_today,
        avg: avg_today_fmt
    };
    let b = Season {
        matches,
        deaths,
        elo_peak: p.season_result.highest.unwrap_or(0),
        elo_lowest: p.season_result.lowest.unwrap_or(0),
        pb: season_best_fmt,
        forfeits: counts.ffs_season,
        forfeit_wins: counts.ff_wins_season,
        slowest: slowest_season_fmt,
        resets: counts.resets_season,
        death_wins: counts.death_wins,
        deathless_wins: counts.deathless_wins,
        wins: p.SeasonStatistics.wins,
        losses: p.SeasonStatistics.losses,
        rank: p.elo_rank.unwrap_or(6969)

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

#[cached(time = 180, sync_writes = "default")]
#[get("/session/<offset>")]
pub async fn session(offset: i64) -> Json<Session> {
    let mut b = Session { ..Default::default() };
    let uuid = UUID.to_string();
    let mh = get_history().await.expect("augh");

    let mut fastest_ms: u32 = 99999999;
    let mut slowest_ms: u32 = 0;
    let mut avg_ms: u32 = 0;

    for m in mh {
        if m.date > offset as u64 {
        let is_win = m.result.uuid.clone().unwrap_or_else(|| "augh".to_string()) == uuid;
        let gd: match_data::GameData = get_match(m.id, m.season).await.unwrap();
            b.matches += 1;
            for plr in gd.changes {
                if plr.uuid == uuid { b.elo += plr.change.unwrap_or(0); }
            }
            if is_win { // win check
                b.wins += 1;
                if !m.forfeited {
                    avg_ms += m.result.time;
                    if fastest_ms > m.result.time { fastest_ms = m.result.time; }
                    if slowest_ms < m.result.time { slowest_ms = m.result.time; }
                } else {
                    b.forfeit_wins += 1; 
                }
            } else if m.result.uuid == Option::None {
                b.draws += 1;
            }
            println!("Match {} S{} in {}m", m.id, m.season, m.result.time/1000/60);
            for timeline in gd.timelines {
                if timeline.uuid != uuid { continue; }
                match timeline.timeline_type.as_str() {
                    "projectelo.timeline.reset" => {
                        b.resets += 1;
                    }
                    "projectelo.timeline.death" => {
                        b.deaths += 1;
                    }
                    "projectelo.timeline.forfeit" => {
                        b.forfeits += 1;
                    }
                    _ => { }
                }
            }
        }
    } // this shit is so ass wilted flower emoji
    b.losses = b.matches - b.wins - b.draws;
    if b.wins == 0 || b.forfeit_wins == b.wins {
        avg_ms = avg_ms / 1;
    } else {
        avg_ms = avg_ms / (b.wins - b.forfeit_wins);
    }
    let slowest_today_fmt = format!("{}:{:02}", slowest_ms / 1000 / 60, slowest_ms / 1000 % 60);
    let avg_today_fmt = format!("{}:{:02}", avg_ms / 1000 / 60, avg_ms / 1000 % 60);
    let fastest_today_fmt: String;
    match fastest_ms {
        99999999 => { fastest_today_fmt = format!("6969:69"); }
        _ => { fastest_today_fmt = format!("{}:{:02}", fastest_ms / 1000 / 60, fastest_ms / 1000 % 60); }
    }
    return Json(Session {
        matches: b.matches,
        deaths: b.deaths,
        elo: b.elo,
        wins: b.wins,
        draws: b.draws,
        losses: b.losses,
        forfeits: b.forfeits,
        forfeit_wins: b.forfeit_wins,
        slowest: slowest_today_fmt,
        fastest: fastest_today_fmt,
        avg: avg_today_fmt,
        resets: b.resets,
    })
}