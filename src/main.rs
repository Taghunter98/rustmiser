mod websoc;

#[tokio::main]
async fn main() {
    if let Err(e) = websoc::run_recipe().await {
        eprintln!("WebSocket error: {}", e);
    }
}
