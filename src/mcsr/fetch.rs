use anyhow::{Error, Result};

use crate::mcsr::*;
// const ID_OFFSET: &str = "5772958"; // 478
const ID_OFFSET: &str = "4815034"; // 1

pub async fn get_match(match_id: u32, season: u8) -> Result<match_data::GameData, Error> {
    let req = format!("https://mcsrranked.com/api/matches/{}?season={}", match_id, season);
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<match_data::Response>().await?;
    Ok(data.data)
}

pub async fn get_history() -> Result<Vec<match_history::GameData>, Error> {
    // -- Season 9 -- //
    // let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4526605"); // 100
    // let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after=4424617"); // 160

    // let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&after={}", ID_OFFSET);
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?count=100&type=2&before={}&sort=oldest", ID_OFFSET);

    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<match_history::Response>().await?;
    Ok(data.data)
}
pub async fn get_slowest() -> Result<Vec<match_history::GameData>, Error> {
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/matches?sort=slowest&count=1");
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<match_history::Response>().await?;
    Ok(data.data)
}

pub async fn get_profile() -> Result<profile_data::Data, Error> {
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc");
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<profile_data::Response>().await?;
    Ok(data.data)
}

pub async fn get_profile_seasons() -> Result<seasons_data::Data, Error> {
    let req = format!("https://mcsrranked.com/api/users/beasttrollmc/seasons");
    let client = reqwest::Client::new();
    let data = client.get(req).send().await?.json::<seasons_data::Response>().await?;
    Ok(data.data)
}