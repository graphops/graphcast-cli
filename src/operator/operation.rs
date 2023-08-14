use chrono::Utc;
use graphcast_sdk::graphql::client_graph_account::subgraph_hash_by_id;
use tracing::{error, info};

use graphcast_sdk::graphcast_agent::GraphcastAgentError;

use crate::messages::upgrade::UpgradeIntentMessage;
use crate::operator::RadioOperator;

impl RadioOperator {
    pub async fn gossip_one_shot(&self) -> Result<String, GraphcastAgentError> {
        // configure radio config to parse in a subcommand for the radio payload message?
        let new_hash = self.config.message().new_hash.clone();
        let subgraph_id = self.config.message().subgraph_id.clone();
        let time = Utc::now().timestamp();
        let graph_account = self.config.graph_stack().graph_account.clone();
        let identifier = subgraph_hash_by_id(
            self.config.graph_stack().network_subgraph(),
            &graph_account,
            &subgraph_id,
        )
        .await?;
        let radio_message =
            UpgradeIntentMessage::build(subgraph_id, new_hash.clone(), time, graph_account);
        match self
            .graphcast_agent
            .send_message(&identifier, radio_message, time)
            .await
        {
            Ok(msg_id) => {
                info!(msg_id, "Sent message");
                Ok(msg_id)
            }
            Err(e) => {
                error!(err = tracing::field::debug(&e), "Failed to send message");
                Err(e)
            }
        }
    }
}
