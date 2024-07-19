use tracing::{debug, info, warn};
use tracing_subscriber::FmtSubscriber;

// Modules avoid declaring with the USEs please
mod config;

#[tokio::main]
async fn main() {
    // This is a debug from BEFORE we start anything. 
    // No point in diagnosing flat crashes from outside the machine
    println!("Initializing Engine");

    // get the logging config
    // We actually use tracing but this is legacy
    let logging_config = config::get_logging_config();
    println!("Logging level: {:?}", logging_config);

    // Set up the tracing subscriber here
    // This is like logging but different. 
    // Some of the crates we use, use tracing so we're stuck with it.
    // Don't ask me to explain it. I dont really understand it very well myself
    let subscriber = FmtSubscriber::builder()
        .with_max_level(logging_config)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");


    info!("test");
}
