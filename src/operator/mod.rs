use std::sync::Arc;
use tracing::debug;

use graphcast_sdk::graphcast_agent::GraphcastAgent;

use crate::config::Config;
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

        RadioOperator {
            config: config.clone(),
            graphcast_agent,
        }
    }

    pub fn graphcast_agent(&self) -> &GraphcastAgent {
        &self.graphcast_agent
    }
}
