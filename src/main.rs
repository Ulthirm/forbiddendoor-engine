mod config;

#[tokio::main]
async fn main() {
    // This is a debug from BEFORE we start anything. 
    // No point in diagnosing flat crashes from outside the machine
    println!("Initializing Engine");

    // using the logging config stuff
    let logging_config = get_logging_config();
    println!("Logging level: {:?}", logging_config);
}
