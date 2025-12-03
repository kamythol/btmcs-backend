use anyhow::Result;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct YouTubeData {
    counters: Counters,
}
#[derive(Deserialize, Serialize, Debug)]
struct Counters {
    estimation: Estimation,
    api: Api,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Estimation {
    subscriber_count: u32,
    view_count: u64,
    video_count: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Api {
    subscriber_count: u32,
    view_count: u64,
    video_count: u32,
}

async fn get_yt_data() -> Result<Counters, anyhow::Error> {
    let req = format!("https://api.socialcounts.org/youtube-live-subscriber-count/UCcHYmDcbHmGIebwGYgLQDOw");
    let client = reqwest::Client::new();
    let data = client
        .get(req)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
        .send()
        .await?
        .json::<YouTubeData>()
        .await?;
    println!("data: {:?}", data);
    Ok(data.counters)
}

#[get("/subs")]
pub async fn get_subs() -> String {
    let data = get_yt_data().await.expect("e");
    return data.estimation.subscriber_count.to_string();  
}

#[get("/views")]
pub async fn get_total_views() -> String {
    let data = get_yt_data().await.expect("e");
    return data.estimation.view_count.to_string()
}

#[get("/videos")]
pub async fn get_total_videos() -> String {
    let data = get_yt_data().await.expect("e");
    return data.estimation.video_count.to_string()
}