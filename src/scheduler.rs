use axum::{Json, extract::rejection::JsonRejection};
use futures_util::StreamExt;
use reqwest::StatusCode;
use serde::Deserialize;
use tokio_cron::Job;

use crate::{GLOBAL_SCHEDULER, scheduler, weather, websoc};

/// Structure Comand holds the data for a NeoHub command.
/// - `cmd` the string command e.g `GET_LIVE_DATA`
/// - `id` the id required e.g 0.
///
#[derive(Deserialize)]
pub struct Command {
    cmd: String,
    id: String,
}

/// Structure Schedule holds the data for schedule.
/// - `run` flag for start/stop cron
/// - `time` cron time e.g `1 * * * *`
/// - `threshold_x` thresholds for recipes.
///
#[derive(Deserialize)]
pub struct Schedule {
    run: bool,
    time: String,
    threshold_1: f64,
    threshold_2: f64,
    threshold_3: f64,
    threshold_4: f64,
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
pub async fn system(payload: Result<Json<Command>, JsonRejection>) -> Json<String> {
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

/// Handler for returning daily minimum temperature.
///
/// ## Errors
///
/// - Returns [`Result<String, Box<dyn std::error::Error>>`](std::error::Error) - if the GET request is unsuccessful.
///
pub async fn temp() -> Result<Json<String>, StatusCode> {
    let min: f64 = weather::get_min_temp("TN174HH", "ac2509f894e84d20b84193300250503").await;

    Ok(Json(format!("The miniumum temperature today is {min}")))
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
pub async fn set_schedule(
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

pub async fn run_recipes_from_data(
    threshold_1: f64,
    threshold_2: f64,
    threshold_3: f64,
    threshold_4: f64,
) {
    let min: f64 = weather::get_min_temp("TN174HH", "ac2509f894e84d20b84193300250503").await;
    println!("The min temp is {min} today");

    // Review min temp
    if min > threshold_1 {
        // 9 deg
        println!("Running 6am Start Time");
        websoc::run("RUN_RECIPE", "['6am Start Time.']").await;
    } else if min > threshold_2 {
        // 5 deg
        println!("Running 4:30am Start Time");
        websoc::run("RUN_RECIPE", "['4.30 am Heating Start']").await;
    } else if min > threshold_3 {
        // 1 deg
        println!("Running 3:30am Start Time");
        websoc::run("RUN_RECIPE", "['3.30am Heating Start.']").await;
    } else if min > threshold_4 {
        // -3 deg
        println!("Running 2am Start Time");
        websoc::run("RUN_RECIPE", "['2am Heating Start.']").await;
    } else {
        println!("Temperature out of range. No recipes run");
    }
}
