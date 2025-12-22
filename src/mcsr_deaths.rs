use cached::proc_macro::cached;
use anyhow::Result;
use tokio::time::{Instant, Duration};

mod match_data;
mod match_history;

async fn get_match(match_id: u32, season: u8) -> Result<match_data::GameData, anyhow::Error> {
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

async fn get_history() -> Result<Vec<match_history::GameData>, anyhow::Error> {
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .send()
        .await?
        .json::<match_history::Response>()
        .await?;
    Ok(data.data)
}

#[cached(time = 300, sync_writes = "default")]
#[get("/deaths")]
pub async fn deaths() -> String {
    let inst = Instant::now();
    let mut deaths: u32 = 0;
    let mut matches: u32 = 0;
    let matchtext = "projectelo.timeline.death".to_string();
    let uuid = "8a8174eb699a49fcb2299af5eede0992".to_string();
    let mh = get_history().await.expect("augh");
    
    for m in mh {
        matches += 1;
        println!("Match {} S{} in {}m", m.id, m.season, m.result.time/1000/60);
        let gd = get_match(m.id, m.season).await.unwrap();
        let timelines = gd.timelines;
        for timeline in timelines {
            if (timeline.timeline_type == matchtext) &&
               (timeline.uuid == uuid) {
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