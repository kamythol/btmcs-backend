#[macro_use] extern crate rocket;
use image::GenericImageView;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio::time::Instant;
use std::collections::HashMap;
use reqwest::Error;
use serde_json::Value;
use std::io::Cursor;
use crate::seed::DailySeed;
use anyhow::Result;

use crate::twitch as Twitch;
pub mod twitch;
pub mod seed;


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

#[get("/emote/<stage>")]
async fn emote(stage: u8) -> Result<(ContentType, Vec<u8>), Status> {
    match emote_image(stage).await {
        Ok(Some(bytes)) => { Ok((ContentType::new("image", "webp"), bytes)) },
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
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
        .json::<Value>().await?.clone() 
    };
    let global_emotes: &Vec<Value> = &data.as_array().expect("e");
    seed.process(global_emotes.len());
    let index = seed.get_index();
    let emote = &global_emotes[index];
    let emote_name = emote["code"].as_str().unwrap().to_string();
    let ge = emote["urls"].as_array().expect("augh2");
    
    let l = ge.iter().find(|url| url["size"] == "4x")
        .and_then(|url| url["url"].as_str())
        .unwrap_or("augh3").to_string();
    println!("[data] {}ms", inst.elapsed().as_millis());

    if what == "link" {
        Ok(l)
    } else if what == "name" {
        Ok(emote_name)
    } else {
        Ok(l)
    }
    
    // let channel_emotes: &Vec<Value> = &data["emote_set"]["emotes"].as_array().expect("augh");
    // seed.process(channel_emotes.len());
    // let random_index = seed.get_index();
    // println!("{}", random_index);
    // println!("{}", set);
    // let emote = &channel_emotes[random_index];
    // let emote_id = emote["id"].as_str().unwrap();
    // let image = format!("https://cdn.7tv.app/emote/{}/4x.webp", emote_id);
    // if what == "link" {
    //     Ok(image)
    // } else if what == "name" {
    //     Ok(emote["name"].as_str().unwrap().to_string())
    // } else {
    //     Ok(image)
    // }
}


#[get("/emotename")]
async fn emotename() -> String {
    let name = get_emote("name").await.unwrap();
    return name
}

#[get("/twitchinfo")]
async fn twitchinfo() -> Json<Vec<Twitch::Channel>> {
    let info = Twitch::is_live("btmc".to_string()).await.unwrap();
    return Json(info)
}

#[get("/followers")]
async fn followers() -> String {
    let followers = Twitch::followers().await.unwrap().to_string();
    return followers
}

#[get("/latest")]
async fn latest() -> Json<Vec<Twitch::Stream>> {
    let streaminfo = Twitch::latest().await.unwrap();
    return Json(streaminfo)
}

#[get("/ping")]
fn ping() -> String {
    return "OK".to_string()
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }.to_cors().expect("cors failed");

    rocket::build()
        .attach(cors)
        // .mount("/random", routes![score])
        // .mount("/real", routes![acc])
        .mount("/random", routes![emote])
        .mount("/real", routes![emotename])
        .mount("/", routes![ping])
        .mount("/", routes![twitchinfo])
        .mount("/twitchinfo", routes![followers])
        .mount("/twitchinfo", routes![latest])
}