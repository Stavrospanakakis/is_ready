use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = is_ready::run().await {
        println!("{}", e);
        process::exit(1);
    }
}
