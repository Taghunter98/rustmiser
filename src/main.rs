
use axum::{
    extract::Path, response::Redirect, routing::get, Error, Json, Router
};
use dotenv::dotenv;
use futures_util::StreamExt;
use reqwest::StatusCode;
use tracing::{Level, info};
use tower_http::services::ServeDir;

mod weather;
mod websoc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    // let min: f64 = weather::get_min_temp("TN174HH", "ac2509f894e84d20b84193300250503").await;
    // println!("The min temp is {min}");

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let static_files: ServeDir = ServeDir::new("./assets");

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/system{cmd}{id}", get(system))
        .route("/temp", get(temp))
        .nest_service("/static", static_files);

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is listening on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> Redirect {
    Redirect::to("/static/index.html")
}

async fn system(Path((cmd, id)): Path<(String, String)>) -> Result<Json<String>, StatusCode> {
    let mut socket = websoc::run(&cmd, &id).await;

    let msg = socket
        .next()
        .await
        .expect("future unable to resolve next item in stream");

    Ok(Json(format!("{:?}", msg)))
}

async fn temp() -> Result<Json<String>, StatusCode> {
    let min: f64 = weather::get_min_temp("TN174HH", "ac2509f894e84d20b84193300250503").await;

    Ok(Json(format!("The miniumum temperature today is {min}")))
}
