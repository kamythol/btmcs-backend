use cached::proc_macro::cached;
use anyhow::{Error, Result};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tokio::time::{Instant, Duration};

mod match_data;
mod match_history;
mod profile_data;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Final {
    deaths: u32,
    elo: u32
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
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4274310");
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
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc?season=9");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .send()
        .await?
        .json::<profile_data::Response>()
        .await?;
    Ok(data.data)
}

#[cached(time = 120, sync_writes = "default")]
#[get("/deaths")]
pub async fn deaths() -> String {
    let inst = Instant::now();
    let mut deaths: u32 = 80;
    let mut matches: u32 = 100;
    let matchtext = "projectelo.timeline.death".to_string();
    let mh = get_history().await.expect("augh");
    
    for m in mh {
        matches += 1;
        println!("Match {} S{} in {}m", m.id, m.season, m.result.time/1000/60);
        let gd = get_match(m.id, m.season).await.unwrap();
        for timeline in gd.timelines {
            if (timeline.timeline_type == matchtext) &&
               (timeline.uuid == UUID.to_string()) {
                println!("{:?}", timeline);
                deaths += 1;
            } else {
                continue;
            }
        }
    }
    let f = format!("{} deaths in {} matches", deaths, matches);
    println!("Took {}ms", inst.elapsed().as_millis());
    return f
}


#[cached(time = 600, sync_writes = "default")]
#[get("/data")]
pub async fn create_data() -> Json<Final>{
    let p = get_profile().await.expect("au");
    let deaths: u32 = 80;
    let a = Final {deaths, elo: p.elo_rate.unwrap_or(0)};
    return Json(a)
}