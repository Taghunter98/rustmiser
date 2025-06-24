use dotenv::dotenv;

mod weather;
mod websoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    websoc::run("GET_SYSTEM", "0")
        .await
        .expect("websocket connection failed");

    let min: f64 = weather::get_min_temp("TN174HH", "ac2509f894e84d20b84193300250503").await;
    println!("The min temp is {min}");
}
