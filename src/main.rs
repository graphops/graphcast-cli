use std::sync::mpsc;

use dotenv::dotenv;
use graphcast_cli::{
    config::{Commands, Config},
    operator::{operation::indexing_status, RadioOperator},
};
use graphcast_sdk::{graphcast_agent::GraphcastAgent, WakuMessage};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Parse basic configurations
    let radio_config = Config::args();

    match &radio_config.subcommand() {
        Commands::UpgradePresync(args) => {
            // The channel is not used in CLI
            let (sender, _) = mpsc::channel::<WakuMessage>();
            let agent = GraphcastAgent::new(
                radio_config.to_graphcast_agent_config().await.unwrap(),
                sender,
            )
            .await
            .expect("Initialize Graphcast agent");

            let radio_operator = RadioOperator::new(&radio_config, agent).await;
            radio_operator.upgrade_presync(args).await;
        }
        Commands::IndexingStatus(args) => {
            // No graphcast agent or radio operator needed
            indexing_status(&radio_config, args).await;
        }
    };
}
