use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .fallback_service(
            ServeDir::new("game/dist").not_found_service(ServeFile::new("game/dist/index.html")),
        )
        .nest_service("/assets", ServeDir::new("game/assets"));
    Ok(router.into())
}
