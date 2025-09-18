use axum::{
    Error, Router,
    routing::{get, post},
};
use dotenv::dotenv;
use futures_util::lock::Mutex;
use once_cell::sync::Lazy;
use tokio_cron::Scheduler;
use tower_http::services::ServeDir;
use tracing::{Level, info};

use crate::ftp::upload;
use crate::scheduler::{set_schedule, system, temp};

mod ftp;
mod scheduler;
mod weather;
mod websoc;

pub static GLOBAL_SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::utc()));

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Serve svelte application via index.html in build
    let static_service = ServeDir::new("build");
    let uploads = ServeDir::new("../uploads");

    // App routes
    let app: Router = Router::new()
        .route("/system", post(system))
        .route("/temp", get(temp))
        .route("/schedule", post(set_schedule))
        .route("/upload", post(upload))
        .nest_service("/uploads", uploads)
        .fallback_service(static_service);

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    info!("Server is listening on http://0.0.0.0:4000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
