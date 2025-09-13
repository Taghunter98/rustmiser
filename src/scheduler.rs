use crate::{weather, websoc};

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
