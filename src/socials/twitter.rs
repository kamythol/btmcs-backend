use scraper::{Html, Selector};

async fn get_profile() -> Result<String, anyhow::Error> {
    let req = format!("https://nitter.tiekoetter.com/btmclive");
    let client = reqwest::Client::new();
    let data = client
        .get(req).send()
        .await?
        .text()
        .await?;

    Ok(data)
}

#[get("/followers")]
pub async fn get_followers() -> String {
    let mut followers = String::new();
    let profile = get_profile().await.unwrap();
    let html = Html::parse_document(&profile);
    let sel = Selector::parse("li.followers span.profile-stat-num").unwrap();

    for e in html.select(&sel) {
        followers.push_str(e.inner_html().as_str());
    }
    followers = followers.replace(",", "");
    return followers
}