// -- unused -- //
use rosu_v2::prelude::*;

async fn get_scores(username: &str) -> Vec<Score> {
    let client_id: u64 = 44609;
    let client_secret = String::from(""); // token
    let osu = Osu::new(client_id, client_secret).await.unwrap();
    
    let scores: Vec<Score> = osu.user_scores(username)
        .mode(GameMode::Osu)
        .best()
        .offset(0)
        .limit(100)
        .await
        .unwrap();
    scores
}

fn get_mods(bits: u32) -> String {
    let mut mods = String::new();

    // Define a mapping from GameMod to their names
    let mod_map: HashMap<GameModsLegacy, &str> = [
        (GameModsLegacy::NoMod, "NM"),
        (GameModsLegacy::NoFail, "NF"),
        (GameModsLegacy::Easy, "EZ"),
        (GameModsLegacy::TouchDevice, "TD"),
        (GameModsLegacy::Hidden, "HD"),
        (GameModsLegacy::SuddenDeath, "SD"),
        (GameModsLegacy::DoubleTime, "DT"),
        (GameModsLegacy::HardRock, "HR"),
        (GameModsLegacy::Relax, "RX"),
        (GameModsLegacy::HalfTime, "HT"),
        (GameModsLegacy::Nightcore, "NC"),
    ].iter().cloned().collect();

    // Check each mod and add it to the vec if it is present
    for (mod_key, &mod_name) in mod_map.iter() {
        if bits & (u32::from(*mod_key)) != 0 {
            mods.push_str(mod_name);
        }
    }
    if mods.len() == 0 {
        mods.push_str("NM");
    }
    mods
}


#[get("/score/<username>")]
async fn score(username: &str) -> String {
    let mut daily_seed = DailySeed::new();
    daily_seed.process(100);
    let scores = get_scores(username).await;
    println!("{}", daily_seed.get_index());
    if let Some(user_score) = scores.get(daily_seed.get_index()) {
        println!("{}", user_score.accuracy);
        if let Some(bm) = user_score.mapset.clone() {
            let pp = user_score.pp.unwrap_or(0.0);
            let score = user_score.legacy_score;
            let lazerstate = user_score.set_on_lazer.to_string();
            let mods = get_mods(user_score.mods.as_legacy().bits());
            let date = user_score.ended_at.to_string();
            let diffname = user_score.map.clone().unwrap().version;
            let stars = user_score.map.clone().unwrap().stars;
            let diff = format!("[{}] ({}, {}*)", diffname, bm.creator_name, stars);
            let map_info = format!("{} - {} {}", bm.artist, bm.title, diff);
                
            let res = format!("{} | {}pp +{} {} lazer score\nSet at {} Lazer: {}", 
                map_info, pp, mods, score, date, lazerstate);
            println!("{}", res);
            return res
        } else {
            return "No map".to_string();
        }
    } else {
        return "No score found.".to_string()
    }
}

#[get("/acc/<username>")]
async fn acc(username: &str) -> String {
    let scores = get_scores(username).await;
    let mut daily_seed = DailySeed::new();
    daily_seed.process(100);
    println!("{}", daily_seed.get_index());
    if let Some(user_score) = scores.get(daily_seed.get_index()) {
        let a: f32 = (user_score.accuracy * 10.0).round() / 10.0;
        return a.to_string()
    } else {
        return "Score not found".to_string()
    }
}