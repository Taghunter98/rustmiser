use dotenv::dotenv;

mod websoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    websoc::run("GET_SYSTEM", "0")
        .await
        .expect("websocket connection failed");
}
