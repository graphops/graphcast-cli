use async_graphql::SimpleObject;
use ethers_contract::EthAbiType;
use ethers_core::types::transaction::eip712::Eip712;
use ethers_derive_eip712::*;
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Eip712, EthAbiType, Clone, Message, Serialize, Deserialize, PartialEq, SimpleObject)]
#[eip712(
    name = "UpgradeIntentMessage",
    version = "0",
    chain_id = 1,
    verifying_contract = "0xc944e90c64b2c07662a292be6244bdf05cda44a7"
)]
pub struct UpgradeIntentMessage {
    /// subgraph id shared by both versions of the subgraph deployment
    #[prost(string, tag = "1")]
    pub subgraph_id: String,
    // new version of the subgraph has a new deployment hash
    #[prost(string, tag = "2")]
    pub new_hash: String,
    /// nonce cached to check against the next incoming message
    #[prost(int64, tag = "3")]
    pub nonce: i64,
    /// Graph account sender - expect the sender to be subgraph owner
    #[prost(string, tag = "4")]
    pub graph_account: String,
}

impl UpgradeIntentMessage {
    pub fn new(subgraph_id: String, new_hash: String, nonce: i64, graph_account: String) -> Self {
        UpgradeIntentMessage {
            subgraph_id,
            new_hash,
            nonce,
            graph_account,
        }
    }

    pub fn build(
        subgraph_id: String,
        new_hash: String,
        timestamp: i64,
        graph_account: String,
    ) -> Self {
        UpgradeIntentMessage::new(subgraph_id, new_hash, timestamp, graph_account)
    }
}
