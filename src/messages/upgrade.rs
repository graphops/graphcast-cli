use async_graphql::SimpleObject;
use ethers_contract::EthAbiType;
use ethers_core::types::transaction::eip712::Eip712;
use ethers_derive_eip712::*;

use graphcast_sdk::networks::NetworkName;
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Eip712, EthAbiType, Clone, Message, Serialize, Deserialize, PartialEq, SimpleObject)]
#[eip712(
    name = "VersionUpgradeMessage",
    version = "0",
    chain_id = 1,
    verifying_contract = "0xc944e90c64b2c07662a292be6244bdf05cda44a7"
)]
pub struct VersionUpgradeMessage {
    // identify through the current subgraph deployment
    #[prost(string, tag = "1")]
    pub identifier: String,
    // new version of the subgraph has a new deployment hash
    #[prost(string, tag = "2")]
    pub new_hash: String,
    /// subgraph id shared by both versions of the subgraph deployment
    #[prost(string, tag = "6")]
    pub subgraph_id: String,
    /// nonce cached to check against the next incoming message
    #[prost(int64, tag = "3")]
    pub nonce: i64,
    /// blockchain relevant to the message
    #[prost(string, tag = "4")]
    pub network: String,
    /// estimated timestamp for the usage to switch to the new version
    #[prost(int64, tag = "5")]
    pub migrate_time: i64,
    /// Graph account sender - expect the sender to be subgraph owner
    #[prost(string, tag = "7")]
    pub graph_account: String,
}

impl VersionUpgradeMessage {
    pub fn new(
        identifier: String,
        new_hash: String,
        subgraph_id: String,
        nonce: i64,
        network: String,
        migrate_time: i64,
        graph_account: String,
    ) -> Self {
        VersionUpgradeMessage {
            identifier,
            new_hash,
            subgraph_id,
            nonce,
            network,
            migrate_time,
            graph_account,
        }
    }

    pub fn build(
        identifier: String,
        new_hash: String,
        timestamp: i64,
        subgraph_id: String,
        network: NetworkName,
        migrate_time: i64,
        graph_account: String,
    ) -> Self {
        VersionUpgradeMessage::new(
            identifier,
            new_hash,
            subgraph_id,
            timestamp,
            network.to_string(),
            migrate_time,
            graph_account,
        )
    }
}
