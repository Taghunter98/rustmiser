use crate::{weather, websoc};

pub async fn run_recipes_from_data() {
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
