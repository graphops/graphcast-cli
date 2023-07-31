use dotenv::dotenv;
use one_shot_cli::{config::Config, operator::RadioOperator, RADIO_OPERATOR};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Parse basic configurations
    let radio_config = Config::args();

    // Initialization and pass in for static lifetime throughout the program
    let radio_operator = RadioOperator::new(&radio_config).await;

    _ = RADIO_OPERATOR.set(radio_operator);

    // Start radio operations
    RADIO_OPERATOR.get().unwrap().run().await;
}
