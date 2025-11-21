#[macro_use] extern crate rocket;
use rocket_cors::{AllowedOrigins, CorsOptions};
use crate::twitch as Twitch;

mod twitch;
mod seed;
mod emotedle;

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
        .mount("/random", routes![emotedle::emote])
        .mount("/real", routes![emotedle::emotename])
        .mount("/", routes![ping])
        .mount("/", routes![Twitch::twitchinfo])
        .mount("/twitchinfo", routes![Twitch::followers])
        .mount("/twitchinfo", routes![Twitch::latest])
}