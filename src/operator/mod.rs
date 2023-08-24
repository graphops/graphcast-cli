use graphcast_sdk::graphql::client_graph_account::subgraph_hash_by_id;
use std::sync::Arc;
use tracing::{debug, warn};

use graphcast_sdk::graphcast_agent::GraphcastAgent;

use crate::config::{Config, UpgradePresyncArg};
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
    pub async fn new(config: &Config, agent: GraphcastAgent) -> RadioOperator {
        debug!("Initializing Graphcast Agent");
        let graphcast_agent = Arc::new(agent);

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
    pub async fn upgrade_presync(&self, args: &UpgradePresyncArg) {
        let mut current_attempt: u64 = 0;

        // Set subscription topic
        let identifier = subgraph_hash_by_id(
            self.config.graph_stack().network_subgraph(),
            &self.config.graph_stack().graph_account,
            &args.subgraph_id,
        )
        .await
        .expect("Failed to match the upgrade intent with an existing subgraph deployment");
        self.graphcast_agent
            .update_content_topics(vec![identifier])
            .await;

        let mut res = self.gossip_one_shot(args).await;
        // Try again if the gossip failed to send while the attempt number is within max_retry
        while res.is_err() && current_attempt < self.config.max_retry {
            warn!(
                err = tracing::field::debug(&res),
                current_attempt, "Failed to gossip, retry"
            );
            current_attempt += 1;
            res = self.gossip_one_shot(args).await;
        }
    }
}
