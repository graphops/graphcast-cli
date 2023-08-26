use graphcast_sdk::graphql::QueryError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, trace};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerUrl {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerInfo {
    pub info: IndexerUrl,
    pub status: IndexingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingStatus {
    pub health: SubgraphHealth,
    pub synced: bool,
    pub chains: Vec<ChainStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(non_camel_case_types)] // Need exact field names to match with GQL response
pub enum SubgraphHealth {
    healthy,
    unhealthy,
    failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainStatus {
    pub network: String,
    pub latest_block: Block,
    pub chain_head_block: Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: String,
    pub hash: String,
}

impl IndexerInfo {
    pub fn latest_block_number(&self) -> i32 {
        self.status.chains[0]
            .latest_block
            .number
            .parse::<i32>()
            .unwrap()
    }

    pub fn chain_head_block_number(&self) -> i32 {
        self.status.chains[0]
            .chain_head_block
            .number
            .parse::<i32>()
            .unwrap()
    }
}

/// Construct indexer url query
pub fn active_indexer_url(subgraph_id: &str) -> serde_json::Value {
    json!({
        "query": r#"query subgraph($id: String!){
            subgraph(id: $id) {
            currentVersion{
              subgraphDeployment{
                indexerAllocations{
                  indexer {
                    id
                    url
                  }
                }
              }
            }
          }
        }"#,
        "variables": {
            "id": subgraph_id.to_string(),
        },
    })
}

/// Construct indexing status query
pub fn status_query(deployment: &str) -> serde_json::Value {
    json!({
        "query": r#"query indexingStatus($subgraphs: [String!]!) {
            indexingStatuses(subgraphs: $subgraphs) {
                subgraph
                health
                synced
                chains {
                    network
                    ... on EthereumIndexingStatus {
                        latestBlock { number hash }
                        chainHeadBlock { number hash }
                    }
                }
            }
        }"#,
        "variables": {
            "subgraphs": [deployment.to_string()],
        },
    })
}

/// Query the network subgraph to get a list of indexer id to their
/// public status endpoint registered by the indexers allocating to the
/// current deployment of the subgraph
pub async fn query_indexer_public_api(
    network_subgraph_endpoint: &str,
    subgraph_id: &str,
) -> Result<Vec<IndexerUrl>, QueryError> {
    // Create GraphQL query string
    let query = active_indexer_url(subgraph_id);

    // Send the GraphQL request
    let response = reqwest::Client::new()
        .post(network_subgraph_endpoint)
        .header("Content-Type", "application/json")
        .json(&query)
        .send()
        .await?;

    // Deserialize the JSON response
    let data: serde_json::Value = response.json().await?;
    let indexer_urls = data["data"]["subgraph"]["currentVersion"]["subgraphDeployment"]
        ["indexerAllocations"]
        .as_array()
        .unwrap_or_else(|| panic!("Could not get indexer public status APIs"))
        .iter()
        .filter_map(|v| serde_json::from_value::<IndexerUrl>(v["indexer"].clone()).ok())
        .collect::<Vec<IndexerUrl>>();
    debug!(
        indexer_urls = tracing::field::debug(&indexer_urls),
        "Queried Indexer URLs"
    );
    Ok(indexer_urls)
}

/// Query the network subgraph to get a hashmap of indexer id to their
/// public status endpoint registered by the indexers allocating to the
/// current deployment of the subgraph
pub async fn query_indexing_status(
    indexer_url: IndexerUrl,
    deployment: String,
) -> Result<IndexerInfo, QueryError> {
    // Create GraphQL query string
    let query = status_query(&deployment);
    let status_endpoint = indexer_url.url.to_string() + "status";
    // Send the GraphQL request
    let response = reqwest::Client::new()
        .post(status_endpoint)
        .header("Content-Type", "application/json")
        .json(&query)
        .send()
        .await?;
    trace!("response: {:#?}", &response);

    // Deserialize the JSON response
    let data: serde_json::Value = response.json().await?;
    let indexing_status =
        serde_json::from_value::<IndexingStatus>(data["data"]["indexingStatuses"][0].clone())
            .map_err(|e| QueryError::Other(e.into()))?;

    debug!(
        indexer = tracing::field::debug(&indexer_url),
        indexing_status = tracing::field::debug(&indexing_status),
        "Queried indexer indexing statuses"
    );
    Ok(IndexerInfo {
        info: indexer_url,
        status: indexing_status,
    })
}

/// Query indexing_status of a deployment indexer_urls
pub async fn query_indexing_statuses(
    indexer_urls: Vec<IndexerUrl>,
    deployment: &str,
) -> Vec<std::result::Result<IndexerInfo, QueryError>> {
    // Loop through indexer_url to query indexing_status
    let indexing_statuses: Vec<_> = indexer_urls
        .iter()
        .map(|indexer_url| query_indexing_status(indexer_url.clone(), deployment.to_string()))
        .collect();
    let mut handles: Vec<tokio::task::JoinHandle<Result<IndexerInfo, QueryError>>> =
        Vec::with_capacity(indexing_statuses.len());

    for fut in indexing_statuses {
        handles.push(tokio::spawn(fut));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    let results_len = results.len();

    let err_results: Vec<&QueryError> = results
        .iter()
        .filter_map(|result| result.as_ref().err())
        .collect();

    debug!(
        results = results_len,
        err_results = err_results.len(),
        "Number of results"
    );
    trace!(err_results = tracing::field::debug(&err_results), "Errors");
    results
}
