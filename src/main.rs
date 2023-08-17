use std::sync::mpsc;

use dotenv::dotenv;
use graphcast_sdk::{graphcast_agent::GraphcastAgent, WakuMessage};
use one_shot_cli::{config::Config, operator::RadioOperator, RADIO_OPERATOR};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Parse basic configurations
    let radio_config = Config::args();

    // The channel is not used in CLI
    let (sender, _) = mpsc::channel::<WakuMessage>();
    let agent = GraphcastAgent::new(
        radio_config.to_graphcast_agent_config().await.unwrap(),
        sender,
    )
    .await
    .expect("Initialize Graphcast agent");
    // Initialization and pass in for static lifetime throughout the program
    let radio_operator = RadioOperator::new(&radio_config, agent).await;

    _ = RADIO_OPERATOR.set(radio_operator);

    // Start radio operations
    RADIO_OPERATOR.get().unwrap().run().await;
}
