use chrono::Utc;
use graphcast_sdk::graphql::client_graph_account::subgraph_hash_by_id;
use graphcast_sdk::graphql::QueryError;
use tracing::{debug, error, info, warn};

use graphcast_sdk::graphcast_agent::GraphcastAgentError;

use crate::config::{Config, IndexingStatusArg, UpgradePresyncArg};
use crate::messages::upgrade::UpgradeIntentMessage;
use crate::operator::RadioOperator;
use crate::query::{query_indexer_public_api, query_indexing_statuses, IndexerInfo};

impl RadioOperator {
    pub async fn gossip_one_shot(
        &self,
        args: &UpgradePresyncArg,
    ) -> Result<String, GraphcastAgentError> {
        // configure radio config to parse in a subcommand for the radio payload message?
        let new_hash = args.new_hash.to_string();
        let subgraph_id = args.subgraph_id.to_string();
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

    /// radio attempt to send message until success or max at configured retry
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

/// Query the new deployment indexing status at public status endpoints registered
/// by the indexers who are actively allocating the current deployment
pub async fn indexing_status(config: &Config, args: &IndexingStatusArg) {
    // Get list of public status APIs
    let public_status_apis = query_indexer_public_api(&config.graph_stack().network_subgraph, &args.subgraph_id).await
        .expect("Could not query public status APIs from indexers actively allocated on the network subgraph");
    info!(
        num_apis = public_status_apis.len(),
        "Number of APIs to query indexing status"
    );

    // Query all the public status APIs for new_hash indexing_status
    let new_hash_statuses = query_indexing_statuses(public_status_apis, &args.new_hash).await;
    debug!("new_hash_statuses {:#?}", new_hash_statuses);
    // Summarize the results
    summarize_indexing_statuses(new_hash_statuses);
}

/// Summarize indexing statuses: Number of indexers,
pub fn summarize_indexing_statuses(statuses: Vec<Result<IndexerInfo, QueryError>>) {
    let okay_results: Vec<IndexerInfo> = statuses
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .cloned()
        .collect();
    let synced_indexers = okay_results
        .iter()
        .filter(|&indexer| indexer.status.synced)
        .count();
    //TODO: to support multiple chains, check chains id specifications, right now brutally taking the first chain
    let block_progress: String = okay_results
        .iter()
        .max_by_key(|indexer| indexer.latest_block_number())
        .map(|indexer| {
            let chain_head = indexer.chain_head_block_number();
            let latest = indexer.latest_block_number();
            format!("{} / {}", chain_head, latest)
        })
        .unwrap_or(String::from("N/A"));
    let num_indexing_indexers = okay_results.len();
    let avg_progress: f32 = okay_results
        .iter()
        .map(|indexer| {
            let progress = indexer.latest_block_number() as f32
                / indexer.chain_head_block_number() as f32
                * 100.0;
            debug!(
                indexer = tracing::field::debug(&indexer.info),
                progress, "Indexer statuses"
            );
            progress
        })
        .sum::<f32>()
        / num_indexing_indexers as f32;
    info!(
        num_currently_allocated_indexer_apis = statuses.len(),
        num_indexing_indexers = num_indexing_indexers,
        num_synced_indexers = synced_indexers,
        latest_synced_block = block_progress,
        average_progress = format!("{}%", avg_progress),
        "Indexing statuses summary"
    );
}
