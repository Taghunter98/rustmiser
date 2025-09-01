use crate::{Schedule, weather, websoc};
use tokio_cron::{Job, Scheduler};

/// Function schedules a cron job to run based of JSON payload recieved.
///
/// If the data is true, then the cron job is run at 11.59 daily or the job is stopped.
pub fn schedule(payload: &axum::Json<Schedule>) -> String {
    println!("Recieved -> set: {} ", payload.run);
    let mut scheduler = Scheduler::local();

    // Run a cron job at given time
    match payload.run {
        true => {
            scheduler.add(Job::named(
                "run_recipes",
                "1 * * * * *",
                run_recipes_from_data,
            ));
            String::from("Running schedule for 11.59pm daily")
        }
        false => {
            Scheduler::cancel_by_name(&mut scheduler, "run_recipes");
            String::from("Stopping schedule for 11.59pm daily")
        }
    }
}

async fn run_recipes_from_data() {
    let min: f64 = weather::get_min_temp("TN174HH", "ac2509f894e84d20b84193300250503").await;
    println!("The min temp is {min} today");

    // Review min temp
    if min > 9.0 {
        println!("Running 6am Start Time");
        websoc::run("RUN_RECIPE", "['6am Start Time.']").await;
    } else if min > 5.0 {
        println!("Running 4:30am Start Time");
        websoc::run("RUN_RECIPE", "['4.30 am Heating Start']").await;
    } else if min > 1.0 {
        println!("Running 3:30am Start Time");
        websoc::run("RUN_RECIPE", "['3.30am Heating Start.']").await;
    } else if min > -3.0 {
        println!("Running 2am Start Time");
        websoc::run("RUN_RECIPE", "['2am Heating Start.']").await;
    } else {
        println!("Temperature out of range. No recipes run");
    }
}
