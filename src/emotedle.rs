use image::GenericImageView;
use anyhow::Result;
use reqwest::Error;
use serde_json::Value;
use std::io::Cursor;
use std::collections::HashMap;
use tokio::time::Instant;
use rocket::http::{ContentType, Status};

use crate::seed::DailySeed;

async fn emote_image(stage: u8) -> anyhow::Result<Option<Vec<u8>>, Error> {
    let inst = Instant::now();
    let mut buf = Vec::new();
    let image_bytes = reqwest::get(get_emote("link").await.expect("failed")).await?.bytes().await?;
    let mut image = image::load(Cursor::new(&image_bytes), image::ImageFormat::WebP).unwrap();
    let x = image.dimensions().0 as f32;
    let y = image.dimensions().1 as f32;
    let stage_map_x: HashMap<u8, f32> = [
        (0, x*0.1),
        (1, x*0.175),
        (2, x*0.25),
        (3, x*0.385),
        (4, x/2.0),
        (5, x*0.75),
        (10, x)
    ].into();
    let stage_map_y: HashMap<u8, f32> = [
        (0, y*0.1),
        (1, y*0.175),
        (2, y*0.25),
        (3, y*0.385),
        (4, y/2.0),
        (5, y*0.75),
        (10, y)
    ].into();
    
    let dim_x = *stage_map_x.get(&stage).unwrap_or(&8.0) as u32;
    let dim_y = *stage_map_y.get(&stage).unwrap_or(&8.0) as u32;
    let n = image.crop((image.dimensions().0 / 2) - (dim_x / 2), (image.dimensions().1 / 2) - (dim_y / 2), dim_x as u32, dim_y);
    n.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::WebP).unwrap();
    println!("[image] {}ms", inst.elapsed().as_millis());
    Ok(Some(buf))
}

async fn get_emote(what: &str) -> anyhow::Result<String, Error> {
    let inst = Instant::now();
    let mut seed = DailySeed::new();
    let set = seed.get_set();
    
    let data: Value = if set == 0 { 
        reqwest::get("https://emotes.crippled.dev/v1/channel/btmc/7tv").await?
        .json().await? 
    } else { 
        reqwest::get("https://emotes.crippled.dev/v1/global/7tv").await?
        .json::<Value>().await?
    };
    let emote_set: &Vec<Value> = &data.as_array().expect("e"); // put json into Vec
    seed.process(emote_set.len());
    let index = seed.get_index();
    let emote = &emote_set[index]; // get a random emote
    let emote_name = emote["code"].as_str().unwrap().to_string(); // get emote name
    let emote_urls = emote["urls"].as_array().expect("augh2"); // grab emote urls into Vec
    
    let link = emote_urls.iter().find(|url| url["size"] == "4x") // get the 4x url
        .and_then(|url| url["url"].as_str())
        .unwrap_or("augh3").to_string();
    println!("[data] {}ms", inst.elapsed().as_millis());

    if what == "link" {
        Ok(link)
    } else if what == "name" {
        Ok(emote_name)
    } else {
        Ok(link)
    }
}


#[get("/emote/<stage>")]
pub async fn emote(stage: u8) -> Result<(ContentType, Vec<u8>), Status> {
    match emote_image(stage).await {
        Ok(Some(bytes)) => { Ok((ContentType::new("image", "webp"), bytes)) },
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/emotename")]
pub async fn emotename() -> String {
    let name = get_emote("name").await.unwrap();
    return name
}