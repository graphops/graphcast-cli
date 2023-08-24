use std::sync::mpsc;

use dotenv::dotenv;
use graphcast_cli::{
    config::{Commands, Config},
    operator::RadioOperator,
};
use graphcast_sdk::{graphcast_agent::GraphcastAgent, WakuMessage};

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

    let radio_operator = RadioOperator::new(&radio_config, agent).await;

    match radio_config.subcommand() {
        Commands::UpgradePresync(args) => {
            radio_operator.upgrade_presync(args).await;
        }
    };
}
