use axum::{
    Error, Json, Router,
    extract::rejection::JsonRejection,
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

    // Serve svelte application via index.html in build
    let static_service: ServeDir = ServeDir::new("build");

    // App routes
    let app: Router = Router::new()
        .route("/system", post(system))
        .route("/temp", get(temp))
        .route("/schedule", post(set_schedule))
        .fallback_service(static_service);

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    info!("Server is listening on http://0.0.0.0:4000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Deserialize)]
struct Command {
    cmd: String,
    id: String,
}

/// Handler returns NeoHub data from command.
///
/// Works by first deserialising the data into a Command struct then a run attempt is made with the
/// new command.
///
/// ## Errors
///
/// - Returns ['JsonRejection'](axum::extract::rejection) - if the JSON data is missing or incorrect.
/// - Resturns ['Json<String>'](axum::json) - if the heatpump rejects the command or is invalid.
///
async fn system(payload: Result<Json<Command>, JsonRejection>) -> Json<String> {
    match payload {
        Ok(payload) => {
            let mut socket = websoc::run(&payload.cmd, &payload.id).await;

            let msg = socket
                .next()
                .await
                .expect("future unable to resolve next item in stream");

            Json(format!("{:?}", msg))
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            Json("Missing JSON content type".to_string())
        }
        Err(_) => Json("Something went wrong".to_string()),
    }
}

async fn temp() -> Result<Json<String>, StatusCode> {
    let min: f64 = weather::get_min_temp("TN174HH", "ac2509f894e84d20b84193300250503").await;

    Ok(Json(format!("The miniumum temperature today is {min}")))
}

#[derive(Deserialize)]
struct Schedule {
    run: bool,
    time: String,
    threshold_1: f64,
    threshold_2: f64,
    threshold_3: f64,
    threshold_4: f64,
}

/// Handler sets the schedule for running recipes at a given time.
///
/// Works by first deserialising the data into a Schedule scruct then runs a new job or attempts to
/// cancel based on job name. The jobs are stored in a global object so that only one instance can be
/// looked up.
///
/// ## Errors
///
/// - Returns ['JsonRejection'](axum::extract::rejection) - if the JSON data is missing or incorrect.
/// - Resturns ['Json<String>'](axum::json) - if the heatpump rejects the command or is invalid.
///
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
                        payload.time.clone(),
                        move || {
                            scheduler::run_recipes_from_data(
                                payload.threshold_1,
                                payload.threshold_2,
                                payload.threshold_3,
                                payload.threshold_4,
                            )
                        },
                    ));
                    data = String::from("Running schedule for 11.59pm daily");
                }
                false => {
                    scheduler.await.cancel_by_name("run_recipes");
                    println!("Canceled job 'run_recipes'");
                    data = String::from("Stopping schedule for 11.59pm daily");
                }
            }

            Json(data.to_string())
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            Json("Missing JSON content type".to_string())
        }
        Err(_) => Json("Something went wrong".to_string()),
    }
}
