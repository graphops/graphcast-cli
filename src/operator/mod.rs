use graphcast_sdk::graphql::client_graph_account::subgraph_hash_by_id;
use std::sync::Arc;
use tracing::{debug, warn};

use graphcast_sdk::graphcast_agent::GraphcastAgent;

use crate::config::Config;
use crate::GRAPHCAST_AGENT;
pub mod operation;

/// Radio operator contains all states needed for radio operations
#[allow(unused)]
pub struct RadioOperator {
    config: Config,
    graphcast_agent: Arc<GraphcastAgent>,
}

impl RadioOperator {
    /// Create a radio operator with radio configurations, persisted data,
    /// graphcast agent, and control flow
    pub async fn new(config: &Config) -> RadioOperator {
        debug!("Initializing Radio operator");
        // Set subscription topic
        let identifier = subgraph_hash_by_id(
            config.graph_stack().network_subgraph(),
            &config.graph_stack().graph_account,
            &config.message().subgraph_id,
        )
        .await
        .expect("Failed to match the upgrade intent with an existing subgraph deployment");

        debug!("Initializing Graphcast Agent");
        let (agent, _receiver) =
            GraphcastAgent::new(config.to_graphcast_agent_config().await.unwrap())
                .await
                .expect("Initialize Graphcast agent");
        let graphcast_agent = Arc::new(agent);
        graphcast_agent
            .update_content_topics(vec![identifier])
            .await;
        debug!("Set global static instance of graphcast_agent");
        _ = GRAPHCAST_AGENT.set(graphcast_agent.clone());

        RadioOperator {
            config: config.clone(),
            graphcast_agent,
        }
    }

    pub fn graphcast_agent(&self) -> &GraphcastAgent {
        &self.graphcast_agent
    }

    /// radio continuously attempt to send message until success
    pub async fn run(&'static self) {
        let mut current_attempt: u64 = 0;
        let mut res = self.gossip_one_shot().await;
        // Try again if the gossip failed to send while the attempt number is within max_retry
        while res.is_err() && current_attempt < self.config.message().max_retry {
            warn!(
                err = tracing::field::debug(&res),
                current_attempt, "Failed to gossip, retry"
            );
            current_attempt += 1;
            res = self.gossip_one_shot().await;
        }
    }
}
