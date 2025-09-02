use axum::{
    Error, Json, Router,
    extract::{Path, rejection::JsonRejection},
    response::Redirect,
    routing::{get, post},
};
use dotenv::dotenv;
use futures_util::{StreamExt, lock::Mutex};
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::Deserialize;
use tokio_cron::{Job, Scheduler};
use tower_http::services::ServeDir;
use tracing::{Level, info};

mod scheduler;
mod weather;
mod websoc;

static GLOBAL_SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::utc()));

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let static_files: ServeDir = ServeDir::new("./assets");

    // App routes
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/system{cmd}/{id}", get(system))
        .route("/temp", get(temp))
        .route("/schedule", post(set_schedule))
        .nest_service("/static", static_files);

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    info!("Server is listening on http://0.0.0.0:4000");
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

#[derive(Deserialize)]
struct Schedule {
    run: bool,
    time: String,
}

async fn set_schedule(
    payload: Result<Json<Schedule>, JsonRejection>,
) -> axum::Json<std::string::String> {
    let data: String;

    match payload {
        Ok(payload) => {
            let scheduler = GLOBAL_SCHEDULER.lock();

            match payload.run {
                true => {
                    scheduler.await.add(Job::named(
                        "run_recipes",
                        &payload.time,
                        scheduler::run_recipes_from_data,
                    ));
                    data = String::from("Running schedule for 11.59pm daily");
                }
                false => {
                    scheduler.await.cancel_by_name("run_recipes");
                    data = String::from("Stopping schedule for 11.59pm daily");
                }
            }

            Json(format!("{}", data))
        }
        Err(JsonRejection::MissingJsonContentType(_)) => Json(format!("Missing JSON content type")),
        Err(_) => Json(format!("Something went wrong")),
    }
}
