use std::env::{current_dir, current_exe};

use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};
async fn hello_world() -> &'static str {
    "Hello world! Go to /game to see your Bevy build."
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let game_dir = current_exe()?.parent().unwrap().join("game");
    let asset_dir = current_dir()?.join("game/assets");
    println!("{}", game_dir.display());
    println!("{}", asset_dir.display());
    let router = Router::new()//.route("/", get(hello_world))
                                      .fallback_service(ServeDir::new(&game_dir)
                                      .not_found_service(ServeFile::new(&game_dir.join("index.html"))))
                                      .nest_service("/assets", ServeDir::new(&asset_dir));
    Ok(router.into())
}