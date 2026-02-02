#[macro_use] extern crate rocket;
use rocket_cors::{AllowedOrigins, CorsOptions};
use crate::{
    socials::twitch as Twitch,
    socials::youtube as YT, 
    socials::twitter as Twitter
};

mod seed;
mod emotedle;
mod mcsr_stats;
mod mcsr;
mod socials {
    pub mod twitch;
    pub mod youtube;
    pub mod twitter;
}

#[get("/ping")]
fn ping() -> String {
    return "ok".to_string()
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }.to_cors().expect("cors failed");

    rocket::build()
        .attach(cors)
        .mount("/", routes![ping])
        .mount("/random", routes![emotedle::emote])
        .mount("/real", routes![emotedle::emotename])
        .mount("/youtube", routes![YT::get_subs])
        .mount("/youtube", routes![YT::get_total_views])
        .mount("/youtube", routes![YT::get_total_videos])
        .mount("/twitch", routes![Twitch::twitchinfo])
        .mount("/twitch", routes![Twitch::followers])
        .mount("/twitch", routes![Twitch::latest_streams])
        .mount("/twitter", routes![Twitter::get_followers])
        .mount("/mcsr", routes![mcsr_stats::deaths])
        .mount("/mcsr", routes![mcsr_stats::create_data])
        .mount("/mcsr", routes![mcsr_stats::session])
}